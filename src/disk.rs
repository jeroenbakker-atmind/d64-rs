//! Disk is a in memory data structure that contains all the data of a single disk.
//!
//! Use [Disk] as the main API entry point.
use std::{
    fs::File,
    io::{Read, Write},
    marker::PhantomData,
    path::Path,
};

use crate::{layout::Layout, Sector, Track};

/// Disk provides a API way how tracks and sectors are logically layed out.
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
    /// The disk layout will be initialized, but the disk isn't formatted yet.
    /// to format the disk check [Disk::format]
    ///
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

    fn initialize_layout(&mut self) {
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

    pub fn write_to_path(&mut self, filename: &Path) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        self.write_to_writer(&mut file)?;
        Ok(())
    }
    pub fn write_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for track in &self.tracks {
            track.write_to_writer(writer)?;
        }
        Ok(())
    }

    /// Get a specific sector of this disk.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// let disk = Disk::<Commodore1541>::new();
    /// let _sector = disk.get_sector(18, 0);
    /// ```
    pub fn get_sector(&self, track_no: u8, sector_no: u8) -> &Sector {
        self.get_track(track_no).get_sector(sector_no)
    }

    fn get_track(&self, track_no: u8) -> &Track {
        let index = (track_no - 1) as usize;
        &self.tracks[index]
    }
    /// Get a specific sector for modification of this disk.
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// let mut _sector = disk.get_sector_mut(18, 0);
    /// ```
    pub fn get_sector_mut(&mut self, track_no: u8, sector_no: u8) -> &mut Sector {
        self.get_track_mut(track_no).get_sector_mut(sector_no)
    }

    fn get_track_mut(&mut self, track_no: u8) -> &mut Track {
        let index = (track_no - 1) as usize;
        &mut self.tracks[index]
    }

    /// Get the name of the disk
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// use std::path::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// let path = Path::new("./disks/1541-empty.d64");
    /// disk.read_from_path(&path).unwrap();
    /// assert_eq!(disk.get_name(), "EMPTY");
    /// ```
    pub fn get_name(&self) -> String {
        L::default().get_disk_name(self)
    }
    /// Set the name of the disk
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// use std::path::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// disk.set_name(&String::from("Hello"));
    /// assert_eq!(disk.get_name(), "HELLO");
    /// ```
    pub fn set_name(&mut self, new_name: &String) {
        L::default().set_disk_name(self, new_name)
    }

    /// Format the disk
    ///
    /// # Example
    ///
    /// ```
    /// use d64::*;
    /// let mut disk = Disk::<Commodore1541>::new();
    /// disk.format();
    /// ```
    pub fn format(&mut self) {
        L::default().format_disk(self);
    }
}
