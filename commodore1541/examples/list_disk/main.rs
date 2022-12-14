use std::path::Path;

use commodore1541::Commodore1541;
use d64::Disk;

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./disks/1541-empty.d64"))?;
    println!("-- {} --", String::from(&disk.get_name()));
    let entries = disk.list_entries();
    for entry in entries {
        println!(
            "{:<3} \"{:<16}\" {:?}",
            entry.num_sectors,
            String::from(&entry.name),
            entry.file_type
        );
    }
    println!("{} free blocks", disk.num_unused_sectors());
    Ok(())
}
