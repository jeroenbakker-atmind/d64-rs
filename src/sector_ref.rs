/// Track number - one based.
pub type TrackNo = u8;
/// Sector number - zero based.
pub type SectorNo = u8;
/// Reference to a single sector on a disk.
/// First element references a track.
/// Second element references a sector within the track.
pub type SectorRef = (TrackNo, SectorNo);
