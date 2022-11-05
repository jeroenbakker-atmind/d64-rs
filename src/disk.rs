use std::{fs::File, marker::PhantomData, path::Path};

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
    pub fn new() -> Self {
        let mut disk = Disk::<L> {
            layout: PhantomData::<L>::default(),
            tracks: Vec::default(),
        };
        disk.initialize_layout();
        disk
    }

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

    pub fn read_from_path(&mut self, filename: &Path) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        self.read_from_file(&mut file)?;
        Ok(())
    }

    // TODO: Should use reader.
    pub fn read_from_file(&mut self, file: &mut File) -> std::io::Result<()> {
        for track in &mut self.tracks {
            track.read_from_file(file)?;
        }
        Ok(())
    }
}
