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
        let sector = disk.get_sector(18, 1);
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
        let sector = disk.get_sector_mut(18, 1);
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

        let sector = disk.get_sector_mut(18, 1);
        // Next sector points to self.
        sector.set_byte(0, 18);
        sector.set_byte(1, 1);
        // Magic bytes.
        sector.set_byte(2, 65);
        sector.set_byte(3, 0);

        // Sectors per track records.
        // and sector availability records.
        let mut offset = 4;
        for track_no in 1..=self.num_tracks() {
            let num_sectors = self.num_sectors(track_no);
            sector.set_byte(offset, num_sectors);
            for sector_no in 1..num_sectors {
                self.mark_sector_free(sector, track_no, sector_no);
            }
            offset += 4;
        }
        self.set_disk_name(disk, &String::from("NONAME"));
    }

    fn clear_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized,
    {
        for track_no in 1..=self.num_tracks() {
            for sector_no in 1..=self.num_sectors(track_no) {
                let sector = disk.get_sector_mut(track_no, sector_no);
                sector.fill(0, self.bytes_per_sector() as usize, 0);
            }
        }
    }
}

impl Commodore1541 {
    fn mark_sector_free(&self, sector: &mut Sector, track_no: u8, sector_no: u8) {
        let offset = (track_no as usize * 4 + sector_no as usize / 8) + 1;
        let shift = sector_no % 8;
        let bit_mask = (1 as u8) << shift;
        let mut availability = *sector.get_byte(offset);
        availability |= bit_mask;
        sector.set_byte(offset, availability);
    }
}
