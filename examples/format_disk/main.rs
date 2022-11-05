use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.initialize_layout();
    disk.format();
    disk.write_to_path(Path::new("./1541-format-test.d64"))?;
    disk.get_sector(18, 1).print();
    Ok(())
}
