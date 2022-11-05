use crate::{decode_petscii, encode_petscii, Disk, Layout, PETSCII_NBSP};

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
}
