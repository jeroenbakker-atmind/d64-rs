use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.initialize_layout();
    disk.read_from_path(Path::new("./disks/1541-empty.d64"))?;
    let sector = disk.get_sector(18, 1);
    sector.print();
    Ok(())
}
