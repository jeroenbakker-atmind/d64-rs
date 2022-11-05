use std::{fs::File, io::Read, marker::PhantomData, path::Path};

use crate::{layout::Layout, Track};

pub struct Disk<L>
where
    L: Layout,
{
    layout: PhantomData<L>,
    tracks: Vec<Track>,
}

impl<L> Disk<L>
where
    L: Layout + Sized + Default,
{
    /// Create a new instance of a disk.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// let _disk = Disk::<Commodore1541>::new();
    /// ```
    pub fn new() -> Self {
        let mut disk = Disk::<L> {
            layout: PhantomData::<L>::default(),
            tracks: Vec::default(),
        };
        disk.initialize_layout();
        disk
    }

    /// Initialize the disk.
    ///
    /// During initialization the layout of the disk is applied.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// disk.initialize_layout();
    /// ```
    pub fn initialize_layout(&mut self) {
        self.tracks.clear();
        let layout = L::default();
        let num_tracks = layout.num_tracks();
        let bytes_per_sector = layout.bytes_per_sector();
        for track_no in 1..=num_tracks {
            let mut track = Track::default();
            let num_sectors = layout.num_sectors(track_no);
            track.initialize(num_sectors, bytes_per_sector);
            self.tracks.push(track);
        }
    }

    /// Load a disk image from file path.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// use std::path::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// let path = Path::new("./disks/1541-empty.d64");
    /// disk.read_from_path(&path).unwrap();
    /// ```
    pub fn read_from_path(&mut self, filename: &Path) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        self.read_from_reader(&mut file)?;
        Ok(())
    }

    /// Load a disk image from a reader.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// use std::path::*;
    /// use std::fs::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// let path = Path::new("./disks/1541-empty.d64");
    /// let mut file = File::open(&path).unwrap();
    /// disk.read_from_reader(&mut file).unwrap();
    /// ```
    pub fn read_from_reader<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        for track in &mut self.tracks {
            track.read_from_reader(reader)?;
        }
        Ok(())
    }
}
