use std::path::Path;

use d64::{commodore1541::Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.format();
    disk.write_to_path(Path::new("./1541-format-test.d64"))?;
    disk.get_sector((18, 0)).print();
    Ok(())
}
