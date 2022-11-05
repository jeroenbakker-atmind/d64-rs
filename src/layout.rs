pub trait Layout {
    fn num_tracks(&self) -> u8;
    fn num_sectors(&self, track: u8) -> u8;
    fn bytes_per_sector(&self) -> u16;
}

pub struct Layout1541 {}

impl Layout for Layout1541 {
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
}
