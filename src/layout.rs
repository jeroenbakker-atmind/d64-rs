use crate::{Disk, PetsciiString, TrackNo};

/// Layout trait to map how a specific device uses its physical media (Disk).
///
/// The layout trait can be implemented by a specific device struct. Common
/// features like tracks, sectors are exposed by this trait to have a common
/// API.
pub trait Layout {
    type FileEntryType;

    /// Number of tracks that are created on the physical media.
    fn num_tracks(&self) -> u8;
    /// Number of sectors that are created on the physical media for a certain track.
    fn num_sectors(&self, track: TrackNo) -> u8;
    /// Bytes that are stored in a single sector.
    fn bytes_per_sector(&self) -> u16;
    /// Extract the human readable name of the given disk.
    fn get_disk_name(&self, disk: &Disk<Self>) -> PetsciiString
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
    fn list_entries(&self, disk: &Disk<Self>) -> Vec<Self::FileEntryType>
    where
        Self: Sized;

    /// Return the contents of the given file.
    fn read_file(&self, disk: &Disk<Self>, file_entry: &Self::FileEntryType) -> Vec<u8>
    where
        Self: Sized;

    fn num_unused_sectors(&self, disk: &mut Disk<Self>) -> usize
    where
        Self: Sized;
}
