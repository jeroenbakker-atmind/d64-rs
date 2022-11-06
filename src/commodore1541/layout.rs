use crate::{
    decode_petscii, encode_petscii, Disk, FileEntry, FileType, Layout, Sector, PETSCII_A,
    PETSCII_NBSP, PETSCII_ONE, PETSCII_TWO, PETSCII_ZERO,
};

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
    fn num_tracks(&self) -> u8 {
        return 35;
    }

    fn num_sectors(&self, track: u8) -> u8 {
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
        return 0;
    }

    fn bytes_per_sector(&self) -> u16 {
        256
    }

    fn get_disk_name(&self, disk: &Disk<Self>) -> String
    where
        Self: Sized,
    {
        let sector = disk.get_sector(18, 0);
        let name_start = 9 * 16;
        let name_end = name_start + 16;
        let mut name = String::new();
        for offset in name_start..name_end {
            let byte = *sector.get_byte(offset);
            if byte == PETSCII_NBSP {
                break;
            }
            name.push(decode_petscii(byte));
        }
        name
    }

    fn set_disk_name(&self, disk: &mut Disk<Self>, new_name: &String)
    where
        Self: Sized,
    {
        let sector = disk.get_sector_mut(18, 0);
        // TODO: max 16 chars.
        let mut name = new_name.clone();
        let name_start = 9 * 16;
        let name_end = name_start + 16;
        sector.fill(name_start, name_end, PETSCII_NBSP);
        let mut name_pos = name_start + new_name.len() - 1;
        while let Some(ch) = name.pop() {
            let byte = encode_petscii(ch, PETSCII_NBSP);
            sector.set_byte(name_pos, byte);
            name_pos -= 1;
        }
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
                let sector = disk.get_sector_mut(track_no, sector_no);
                sector.fill(0, self.bytes_per_sector() as usize, 0);
            }
        }
    }

    fn list_entries(&self, disk: &Disk<Self>) -> Vec<FileEntry>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        let mut sector = disk.get_sector(18, 0);
        while let Some(s) = self.get_next_sector(disk, sector) {
            sector = s;

            let mut entry_bytes = [0_u8; 16];
            for sector_entry in 0..8 {
                sector.get_bytes(sector_entry * 32, &mut entry_bytes);
                let entry = FileEntry::from_bytes(&entry_bytes);
                if entry.file_type != FileType::Scratched {
                    result.push(entry);
                }
            }
        }
        result
    }
}

impl Commodore1541 {
    fn mark_sector_unused(&self, disk: &mut Disk<Self>, track_no: u8, sector_no: u8) {
        let sector = disk.get_sector_mut(18, 0);
        let track_offset = track_no as usize * 4;
        let sector_offset = (track_offset + sector_no as usize / 8) + 1;
        let shift = sector_no % 8;
        let bit_mask = (1 as u8) << shift;
        let availability = *sector.get_byte(sector_offset);
        let new_availability = availability | bit_mask;
        sector.set_byte(sector_offset, new_availability);
        if availability != new_availability {
            let sectors_free = *sector.get_byte(track_offset);
            sector.set_byte(track_offset, sectors_free + 1);
        }
    }

    fn mark_sector_used(&self, disk: &mut Disk<Self>, track_no: u8, sector_no: u8) {
        let sector = disk.get_sector_mut(18, 0);
        let track_offset = track_no as usize * 4;
        let sector_offset = (track_offset + sector_no as usize / 8) + 1;
        let shift = sector_no % 8;
        let bit_mask = (1 as u8) << shift;
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
        let sector = disk.get_sector_mut(18, 0);
        for offset in 160..171 {
            sector.set_byte(offset, PETSCII_NBSP);
        }
        sector.set_byte(162, PETSCII_ZERO);
        sector.set_byte(163, PETSCII_ONE);
        sector.set_byte(165, PETSCII_TWO);
        sector.set_byte(166, PETSCII_A);
    }

    fn initialize_dos_version(&self, disk: &mut Disk<Self>) {
        let sector = disk.get_sector_mut(18, 0);
        sector.set_byte(2, 65);
    }

    fn initialize_bam(&self, disk: &mut Disk<Self>) {
        for track_no in 1..=self.num_tracks() {
            let num_sectors = self.num_sectors(track_no);
            for sector_no in 0..num_sectors {
                self.mark_sector_unused(disk, track_no, sector_no);
            }
        }
        self.mark_sector_used(disk, 18, 0);
    }

    fn initialize_directory_listing(&self, disk: &mut Disk<Self>) {
        let sector180 = disk.get_sector_mut(18, 0);
        self.set_next_sector(sector180, 18, 1);

        let sector181 = disk.get_sector_mut(18, 1);
        self.end_sector_chain(sector181);
    }

    /// Set the next sector for the given sector in a chain of sectors.
    fn set_next_sector(&self, sector: &mut Sector, track_no: u8, sector_no: u8) {
        sector.set_byte(0, track_no);
        sector.set_byte(1, sector_no);
    }

    fn get_next_sector<'a>(&self, disk: &'a Disk<Self>, sector: &Sector) -> Option<&'a Sector> {
        let track_no = *sector.get_byte(0);
        let sector_no = *sector.get_byte(1);
        if track_no == 0 && sector_no == 255 {
            None
        } else {
            Some(disk.get_sector(track_no, sector_no))
        }
    }

    /// Mark the given sector to be the last sector in a chain.
    fn end_sector_chain(&self, sector: &mut Sector) {
        self.set_next_sector(sector, 0, 255);
    }
}
