use crate::{decode_petscii, encode_petscii, Disk, Layout, Sector, PETSCII_NBSP};

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

        let sector = disk.get_sector_mut(18, 0);
        // Next sector (disk listing)
        sector.set_byte(0, 18);
        sector.set_byte(1, 1);
        // DOS Version
        sector.set_byte(2, 65);
        // Unused.
        sector.set_byte(3, 0);

        // Per track sector availability
        for track_no in 1..=self.num_tracks() {
            let num_sectors = self.num_sectors(track_no);
            for sector_no in 0..num_sectors {
                self.mark_sector_free(sector, track_no, sector_no);
            }
        }
        self.mark_sector_used(sector, 18, 0);

        // Initialize default ID (01-2A)
        // Disk ID is 11 chars.
        for offset in 160..171 {
            sector.set_byte(offset, PETSCII_NBSP);
        }
        sector.set_byte(162, encode_petscii('0', PETSCII_NBSP));
        sector.set_byte(163, encode_petscii('1', PETSCII_NBSP));
        sector.set_byte(165, encode_petscii('2', PETSCII_NBSP));
        sector.set_byte(166, encode_petscii('A', PETSCII_NBSP));

        self.set_disk_name(disk, &String::from("NONAME"));
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
}

impl Commodore1541 {
    fn mark_sector_free(&self, sector: &mut Sector, track_no: u8, sector_no: u8) {
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

    fn mark_sector_used(&self, sector: &mut Sector, track_no: u8, sector_no: u8) {
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
}
