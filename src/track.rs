use crate::Sector;

#[derive(Default)]
pub struct Track{
    sectors: Vec<Sector>
}

impl Track {
    pub fn initialize(&mut self, num_sectors: u8, bytes_per_sector: u16) {
        self.sectors.clear();
        for _sector_no in 1..=num_sectors {
            let mut sector = Sector::default();
            sector.initialize(bytes_per_sector as usize);
            self.sectors.push(sector);
        }
    }
}

