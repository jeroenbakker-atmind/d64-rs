use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./triad_continuum.d64"))?;
    println!("-- {} --", String::from(&disk.get_name()));
    let entries = disk.list_entries();
    for entry in entries {
        println!(
            "{:<3} \"{:<16}\" {:?}",
            entry.num_blocks,
            String::from(&entry.name),
            entry.file_type
        );
    }
    Ok(())
}
