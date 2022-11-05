use crate::Disk;

pub trait Layout {
    fn num_tracks(&self) -> u8;
    fn num_sectors(&self, track: u8) -> u8;
    fn bytes_per_sector(&self) -> u16;
    fn get_disk_name(&self, disk: &Disk<Self>) -> String
    where
        Self: Sized;
    fn set_disk_name(&self, disk: &mut Disk<Self>, new_name: &String)
    where
        Self: Sized;
    fn format_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized;

    /// Set all content of the disk to 0. (each track, sector, byte)
    fn clear_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized;
}
