pub trait Layout {
    fn num_tracks(&self) -> u8;
    fn num_sectors(&self, track: u8) -> u8;
    fn bytes_per_sector(&self) -> u16;
}
