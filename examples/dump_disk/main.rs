use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./disks/1541-empty.d64"))?;
    println!("-- {} --", disk.get_name());
    let sector = disk.get_sector(18, 0);
    sector.print();
    Ok(())
}
