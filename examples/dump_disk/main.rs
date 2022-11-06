use std::path::Path;

use d64::{Commodore1541, Disk};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.read_from_path(Path::new("./disks/1541-empty.d64"))?;
    println!("-- {} --", disk.get_name());
    for sector_no in 0..2 {
        println!(" -- 18-{:02} --", sector_no);
        let sector = disk.get_sector((18, sector_no));
        sector.print();
    }
    Ok(())
}
