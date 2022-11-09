use std::io::{Read, Write};

use crate::Sector;

#[derive(Default)]
pub struct Track {
    sectors: Vec<Sector>,
}

impl Track {
    pub fn initialize(&mut self, num_sectors: u8, bytes_per_sector: u16) {
        self.sectors.clear();
        for _sector_no in 0..num_sectors {
            let mut sector = Sector::default();
            sector.initialize(bytes_per_sector as usize);
            self.sectors.push(sector);
        }
    }

    pub fn read_from_reader<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        for sector in &mut self.sectors {
            sector.read_from_reader(reader)?;
        }
        Ok(())
    }
    pub fn write_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for sector in &self.sectors {
            sector.write_to_writer(writer)?;
        }
        Ok(())
    }

    pub fn get_sector(&self, sector_no: u8) -> &Sector {
        let index = sector_no as usize;
        &self.sectors[index]
    }
    pub fn get_sector_mut(&mut self, sector_no: u8) -> &mut Sector {
        let index = sector_no as usize;
        &mut self.sectors[index]
    }
}
