use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./triad_continuum.d64"))?;
    println!("-- {} --", disk.get_name());
    println!("{:#?}", disk.list_entries());
    Ok(())
}
