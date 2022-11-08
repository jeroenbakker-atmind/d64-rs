use crate::{
    BlockAvailabilityMap, Disk, FileEntry, FileListEntryRef, FileType, Layout, PetsciiString,
    Sector, SectorRef, TrackNo, PETSCII_A, PETSCII_NBSP, PETSCII_ONE, PETSCII_TWO, PETSCII_ZERO,
};

/// Track number containing info about the disk, and files on the disk.
const TRACK_HEADER: TrackNo = 18;
/// Reference to the sector containing the BAM, disk name and disk id.
const SECTOR_DISK_HEADER: SectorRef = (TRACK_HEADER, 0);
/// Default sector where to start the file list stored on the disk.
const SECTOR_DISK_LISTING: SectorRef = (TRACK_HEADER, 1);
/// Indicates that marks the end of a chain of sectors.
const SECTOR_END_OF_CHAIN: SectorRef = (0, 255);
/// Header of a sector is 2 bytes. It contains the sector ref to the next sector, or SECTOR_END_OF_CHAIN for the last.
const SECTOR_HEADER_SIZE: usize = 2;
/// Size of actual content that can be stored in a single sector.
const CONTENT_BYTES_PER_SECTOR: usize = 254;

const FILE_LIST_ENTRY_SIZE: usize = 32;
/// Commodore 1541 disk-drive.
///
/// Contains all the logic how a Commodore 1541 (and compatible) disk drives
/// store its data on a disk. It only supports the standard layout.
///
/// A Commodore 1541 can be programmed to store its data differently on the
/// physical media. This programming isn't supported. When needed you have to
/// implement your own Layout.
#[derive(Default)]
pub struct Commodore1541 {}

impl Layout for Commodore1541 {
    /// Type to contain a single entry for [Layout::list_entries].
    type FileEntryType = FileEntry;

    fn num_tracks(&self) -> u8 {
        35
    }

    fn num_sectors(&self, track: TrackNo) -> u8 {
        if (1..=17).contains(&track) {
            return 21;
        }
        if (18..=24).contains(&track) {
            return 19;
        }
        if (25..=30).contains(&track) {
            return 18;
        }
        if (31..=35).contains(&track) {
            return 17;
        }
        0
    }

    fn bytes_per_sector(&self) -> u16 {
        256
    }

    fn get_disk_name(&self, disk: &Disk<Self>) -> PetsciiString
    where
        Self: Sized,
    {
        let sector = disk.get_sector(SECTOR_DISK_HEADER);
        let name_start = 9 * 16;
        let mut bytes = [0_u8; 16];
        sector.get_bytes(name_start, &mut bytes);
        PetsciiString::fixed_size(&bytes)
    }

    fn set_disk_name(&self, disk: &mut Disk<Self>, new_name: &String)
    where
        Self: Sized,
    {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        // TODO: max 16 chars.
        let petscii_string = PetsciiString::from(new_name);
        let bytes = petscii_string.bytes.as_slice();
        let name_start = 9 * 16;
        let name_end = name_start + 16;
        sector.fill(name_start, name_end, PETSCII_NBSP);
        sector.set_bytes(name_start, bytes);
    }

    fn format_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized,
    {
        self.clear_disk(disk);
        self.initialize_dos_version(disk);
        self.initialize_bam(disk);
        self.set_disk_name(disk, &String::from("NONAME"));
        self.initialize_disk_id(disk);
        self.initialize_directory_listing(disk);
    }

    fn clear_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized,
    {
        for track_no in 1..=self.num_tracks() {
            for sector_no in 0..self.num_sectors(track_no) {
                let sector = disk.get_sector_mut((track_no, sector_no));
                sector.fill(0, self.bytes_per_sector() as usize, 0);
            }
        }
    }

    fn list_entries(&self, disk: &Disk<Self>) -> Vec<FileEntry>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        let mut sector = disk.get_sector(SECTOR_DISK_HEADER);
        while let Some(s) = self.get_next_sector(disk, sector) {
            sector = s.0;

            let mut entry_bytes = [0_u8; FILE_LIST_ENTRY_SIZE];
            for sector_entry in 0..8 {
                sector.get_bytes(sector_entry * FILE_LIST_ENTRY_SIZE, &mut entry_bytes);
                let entry = FileEntry::from_bytes(&entry_bytes);
                if entry.file_type != FileType::Scratched {
                    result.push(entry);
                }
            }
        }
        result
    }

    /// Return the contents of the given file.
    fn read_file(&self, disk: &Disk<Self>, file_entry: &FileEntry) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        self.read_sector_chain(disk, file_entry.start_sector, &mut result);

        result
    }

    /// Create a new file and store it to disk.
    fn create_file(&self, disk: &mut Disk<Self>, file_entry: &Self::FileEntryType, content: &[u8])
    where
        Self: Sized,
    {
        let chunks = content.chunks(CONTENT_BYTES_PER_SECTOR);
        let num_sectors = chunks.len();
        if let Some(sectors) = self.allocate_sectors(disk, num_sectors) {
            for (sector_ref, chunk) in sectors.iter().zip(chunks) {
                let sector = disk.get_sector_mut(*sector_ref);
                sector.set_bytes(SECTOR_HEADER_SIZE, chunk);
            }

            let mut file_entry = file_entry.clone();
            file_entry.start_sector = sectors[0];
            file_entry.num_blocks = num_sectors;
            self.create_file_list_entry(disk, &file_entry);
        }
    }

    fn num_unused_sectors(&self, disk: &mut Disk<Self>) -> usize
    where
        Self: Sized,
    {
        let bam = self.get_block_availability_map(disk);
        bam.count_unused_sectors(1, self.num_tracks())
            - bam.count_unused_track_sectors(TRACK_HEADER) as usize
    }
}

impl Commodore1541 {
    fn get_block_availability_map<'a>(&self, disk: &'a mut Disk<Self>) -> BlockAvailabilityMap<'a> {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        BlockAvailabilityMap::new(sector)
    }

    fn mark_sector_used(&self, disk: &mut Disk<Self>, sector_ref: SectorRef) {
        let mut bam = self.get_block_availability_map(disk);
        bam.mark_used(sector_ref);
    }

    // Initialize the disk ID default=01-2A
    fn initialize_disk_id(&self, disk: &mut Disk<Self>) {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        for offset in 160..171 {
            sector.set_byte(offset, PETSCII_NBSP);
        }
        sector.set_byte(162, PETSCII_ZERO);
        sector.set_byte(163, PETSCII_ONE);
        sector.set_byte(165, PETSCII_TWO);
        sector.set_byte(166, PETSCII_A);
    }

    fn initialize_dos_version(&self, disk: &mut Disk<Self>) {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        sector.set_byte(2, 65);
    }

    fn initialize_bam(&self, disk: &mut Disk<Self>) {
        let mut bam = self.get_block_availability_map(disk);
        for track_no in 1..=self.num_tracks() {
            let num_sectors = self.num_sectors(track_no);
            for sector_no in 0..num_sectors {
                bam.mark_unused((track_no, sector_no));
            }
        }
        bam.mark_used(SECTOR_DISK_HEADER);
    }

    fn initialize_directory_listing(&self, disk: &mut Disk<Self>) {
        let sector180 = disk.get_sector_mut(SECTOR_DISK_HEADER);
        self.set_next_sector(sector180, SECTOR_DISK_LISTING);

        let sector181 = disk.get_sector_mut(SECTOR_DISK_LISTING);
        self.end_sector_chain(sector181);
        self.mark_sector_used(disk, SECTOR_DISK_LISTING);
    }

    /// Set the next sector for the given sector in a chain of sectors.
    fn set_next_sector(&self, sector: &mut Sector, sector_ref: SectorRef) {
        sector.set_byte(0, sector_ref.0);
        sector.set_byte(1, sector_ref.1);
    }

    fn get_next_sector<'a>(
        &self,
        disk: &'a Disk<Self>,
        sector: &Sector,
    ) -> Option<(&'a Sector, SectorRef)> {
        let track_no = *sector.get_byte(0);
        if track_no == SECTOR_END_OF_CHAIN.0 {
            None
        } else {
            let sector_no = *sector.get_byte(1);
            let sector_ref = (track_no, sector_no);
            Some((disk.get_sector(sector_ref), sector_ref))
        }
    }

    /// Mark the given sector to be the last sector in a chain.
    fn end_sector_chain(&self, sector: &mut Sector) {
        self.set_next_sector(sector, SECTOR_END_OF_CHAIN);
    }

    fn read_sector_chain(
        &self,
        disk: &Disk<Self>,
        sector_ref: SectorRef,
        file_content: &mut Vec<u8>,
    ) {
        const CONTENT_BYTES_PER_SECTOR: usize = 254;
        if sector_ref.0 == SECTOR_END_OF_CHAIN.0 {
            return;
        }
        let mut bytes = [0_u8; CONTENT_BYTES_PER_SECTOR];

        let mut sector = disk.get_sector(sector_ref);
        sector.get_bytes(2, &mut bytes);
        file_content.extend_from_slice(&bytes);

        while let Some(s) = self.get_next_sector(disk, sector) {
            sector = s.0;
            sector.get_bytes(2, &mut bytes);
            file_content.extend_from_slice(&bytes);
        }
    }

    fn chain_sectors(&self, disk: &mut Disk<Self>, sectors: &[SectorRef]) {
        if sectors.is_empty() {
            return;
        }

        for i in 0..sectors.len() - 1 {
            let sector_ref = sectors[i];
            let next_sector_ref = sectors[i + 1];
            let sector = disk.get_sector_mut(sector_ref);
            self.set_next_sector(sector, next_sector_ref);
        }
        let sector = disk.get_sector_mut(*sectors.last().unwrap());
        self.end_sector_chain(sector);
    }

    fn allocate_sectors(
        &self,
        disk: &mut Disk<Self>,
        num_sectors: usize,
    ) -> Option<Vec<SectorRef>> {
        if num_sectors > self.num_unused_sectors(disk) {
            None
        } else {
            let mut bam = self.get_block_availability_map(disk);
            let sectors = bam.allocate_sectors(num_sectors);
            if let Some(sectors) = &sectors {
                self.chain_sectors(disk, &sectors);
            }
            sectors
        }
    }

    fn create_file_list_entry(&self, disk: &mut Disk<Self>, file_entry: &FileEntry) {
        if let Some(entry_ref) = self.find_scratched_file_list_entry(disk) {
            self.update_file_list_entry(disk, entry_ref, file_entry);
        }
    }

    fn find_scratched_file_list_entry(&self, disk: &mut Disk<Self>) -> Option<FileListEntryRef> {
        let mut sector = disk.get_sector(SECTOR_DISK_HEADER);
        while let Some(s) = self.get_next_sector(disk, sector) {
            sector = s.0;
            let sector_ref = s.1;

            let mut entry_bytes = [0_u8; FILE_LIST_ENTRY_SIZE];
            for sector_entry in 0..8 {
                sector.get_bytes(sector_entry * FILE_LIST_ENTRY_SIZE, &mut entry_bytes);
                let entry = FileEntry::from_bytes(&entry_bytes);
                if entry.file_type == FileType::Scratched {
                    return Some((sector_ref, sector_entry));
                }
            }
        }
        None
    }

    fn update_file_list_entry(
        &self,
        disk: &mut Disk<Self>,
        entry_ref: FileListEntryRef,
        file_entry: &FileEntry,
    ) {
        let sector = disk.get_sector_mut(entry_ref.0);
        let offset = entry_ref.1 * FILE_LIST_ENTRY_SIZE;
        file_entry.store(sector, offset);
    }
}
