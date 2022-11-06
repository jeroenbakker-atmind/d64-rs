use crate::{Disk, FileEntry};

/// Layout trait to map how a specific device uses its physical media (Disk).
///
/// The layout trait can be implemented by a specific device struct. Common
/// features like tracks, sectors are exposed by this trait to have a common
/// API.
pub trait Layout {
    /// Number of tracks that are created on the physical media.
    fn num_tracks(&self) -> u8;
    /// Number of sectors that are created on the physical media for a certain track.
    fn num_sectors(&self, track: u8) -> u8;
    /// Bytes that are stored in a single sector.
    fn bytes_per_sector(&self) -> u16;
    /// Extract the human readable name of the given disk.
    fn get_disk_name(&self, disk: &Disk<Self>) -> String
    where
        Self: Sized;
    /// Change the human readable name of the given disk.
    fn set_disk_name(&self, disk: &mut Disk<Self>, new_name: &String)
    where
        Self: Sized;
    /// Format the disk.
    fn format_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized;

    /// Set all content of the disk to 0. (each track, sector, byte)
    fn clear_disk(&self, disk: &mut Disk<Self>)
    where
        Self: Sized;

    /// List all file entries of the given disk.
    // TODO: Use trait typing.
    fn list_entries(&self, disk: &Disk<Self>) -> Vec<FileEntry>
    where
        Self: Sized;
}
