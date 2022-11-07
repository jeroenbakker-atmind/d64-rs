use crate::{Sector, SectorRef, TrackNo};

pub struct BlockAvailabilityMap<'a> {
    sector: &'a mut Sector,
}

fn get_track_offset(sector: SectorRef) -> usize {
    sector.0 as usize * 4
}

fn get_sector_offset(sector: SectorRef) -> usize {
    get_track_offset(sector) + (sector.1 as usize / 8) + 1
}

fn get_sector_mask(sector: SectorRef) -> u8 {
    sector.1 % 8
}

impl<'a> BlockAvailabilityMap<'a> {
    pub fn new(sector: &'a mut Sector) -> BlockAvailabilityMap<'a> {
        BlockAvailabilityMap { sector }
    }

    pub fn mark_used(&mut self, sector: SectorRef) {
        let track_offset = get_track_offset(sector);
        let sector_offset = get_sector_offset(sector);
        let bit_mask = get_sector_mask(sector);

        let availability = *self.sector.get_byte(sector_offset);
        let new_availability = availability & (255 - bit_mask);
        self.sector.set_byte(sector_offset, new_availability);

        if availability != new_availability {
            let sectors_free = *self.sector.get_byte(track_offset);
            self.sector.set_byte(track_offset, sectors_free - 1);
        }
    }

    pub fn mark_unused(&mut self, sector: SectorRef) {
        let track_offset = get_track_offset(sector);
        let sector_offset = get_sector_offset(sector);
        let bit_mask = get_sector_mask(sector);

        let availability = *self.sector.get_byte(sector_offset);
        let new_availability = availability | bit_mask;
        self.sector.set_byte(sector_offset, new_availability);

        if availability != new_availability {
            let sectors_free = *self.sector.get_byte(track_offset);
            self.sector.set_byte(track_offset, sectors_free + 1);
        }
    }

    pub fn count_unused_track_sectors(&self, track_no: TrackNo) -> u8 {
        let track_offset = get_track_offset((track_no, 0));
        *self.sector.get_byte(track_offset)
    }

    pub fn count_unused_sectors(&self, min_track_no: TrackNo, max_track_no: TrackNo) -> usize {
        let mut result = 0;
        for track_no in min_track_no..=max_track_no {
            result += self.count_unused_track_sectors(track_no) as usize;
        }
        result
    }
}
