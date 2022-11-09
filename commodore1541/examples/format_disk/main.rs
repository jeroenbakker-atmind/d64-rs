use std::path::Path;

use commodore1541::{debug::print_sector, Commodore1541};
use d64::Disk;

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.format();
    disk.write_to_path(Path::new("./1541-format-test.d64"))?;
    print_sector(disk.get_sector((18, 0)));
    Ok(())
}
