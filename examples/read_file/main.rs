use std::path::Path;

use d64::{Commodore1541, Disk, FileType};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./triad_continuum.d64"))?;
    println!("-- {} --", disk.get_name());
    let entries = disk.list_entries();
    for entry in entries {
        if entry.file_type != FileType::Program {
            continue;
        }
        let content = disk.read_file(&entry);
        println!(
            "Read {} bytes of file \"{}\"",
            content.len(),
            String::from(&entry.name)
        );
        break;
    }
    Ok(())
}
