use crate::{
    Disk, FileEntry, FileType, Layout, PetsciiString, Sector, SectorRef, TrackNo, PETSCII_A,
    PETSCII_NBSP, PETSCII_ONE, PETSCII_TWO, PETSCII_ZERO,
};

/// Reference to the sector containing the BAM, disk name and disk id.
const SECTOR_DISK_HEADER: SectorRef = (18, 0);
/// Default sector where to start the file list stored on the disk.
const SECTOR_DISK_LISTING: SectorRef = (18, 1);
/// Indicates that marks the end of a chain of sectors.
const SECTOR_END_OF_CHAIN: SectorRef = (0, 255);

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
        const BYTES_PER_ENTRY: usize = 32;
        let mut result = Vec::new();
        let mut sector = disk.get_sector(SECTOR_DISK_HEADER);
        while let Some(s) = self.get_next_sector(disk, sector) {
            sector = s;

            let mut entry_bytes = [0_u8; BYTES_PER_ENTRY];
            for sector_entry in 0..8 {
                sector.get_bytes(sector_entry * BYTES_PER_ENTRY, &mut entry_bytes);
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
}

impl Commodore1541 {
    fn mark_sector_unused(&self, disk: &mut Disk<Self>, sector_ref: SectorRef) {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        let track_offset = sector_ref.0 as usize * 4;
        let sector_offset = (track_offset + sector_ref.1 as usize / 8) + 1;
        let shift = sector_ref.1 % 8;
        let bit_mask = (1_u8) << shift;
        let availability = *sector.get_byte(sector_offset);
        let new_availability = availability | bit_mask;
        sector.set_byte(sector_offset, new_availability);
        if availability != new_availability {
            let sectors_free = *sector.get_byte(track_offset);
            sector.set_byte(track_offset, sectors_free + 1);
        }
    }

    fn mark_sector_used(&self, disk: &mut Disk<Self>, sector_ref: SectorRef) {
        let sector = disk.get_sector_mut(SECTOR_DISK_HEADER);
        let track_offset = sector_ref.0 as usize * 4;
        let sector_offset = (track_offset + sector_ref.1 as usize / 8) + 1;
        let shift = sector_ref.1 % 8;
        let bit_mask = (1_u8) << shift;
        let availability = *sector.get_byte(sector_offset);
        let new_availability = availability & (255 - bit_mask);
        sector.set_byte(sector_offset, new_availability);
        if availability != new_availability {
            let sectors_free = *sector.get_byte(track_offset);
            sector.set_byte(track_offset, sectors_free - 1);
        }
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
        for track_no in 1..=self.num_tracks() {
            let num_sectors = self.num_sectors(track_no);
            for sector_no in 0..num_sectors {
                self.mark_sector_unused(disk, (track_no, sector_no));
            }
        }
        self.mark_sector_used(disk, SECTOR_DISK_HEADER);
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

    fn get_next_sector<'a>(&self, disk: &'a Disk<Self>, sector: &Sector) -> Option<&'a Sector> {
        let track_no = *sector.get_byte(0);
        if track_no == SECTOR_END_OF_CHAIN.0 {
            None
        } else {
            let sector_no = *sector.get_byte(1);
            let sector_ref = (track_no, sector_no);
            Some(disk.get_sector(sector_ref))
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
            sector = s;
            sector.get_bytes(2, &mut bytes);
            file_content.extend_from_slice(&bytes);
        }
    }
}
