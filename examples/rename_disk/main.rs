use std::path::Path;

use d64::{commodore1541::Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./disks/1541-empty.d64"))?;
    println!("-- old name: {} --", String::from(&disk.get_name()));
    disk.set_name(&String::from("TEST"));
    println!("-- new name: {} --", String::from(&disk.get_name()));
    disk.write_to_path(Path::new("./1541-rename-test.d64"))?;
    Ok(())
}
