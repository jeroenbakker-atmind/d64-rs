use d64::{Commodore1541, Disk, FileEntry, PetsciiString};

fn main() -> std::io::Result<()> {
    let mut disk = Disk::<Commodore1541>::new();
    disk.format();
    let entry = FileEntry {
        name: PetsciiString::from(&String::from("HELLO WORLD")),
        file_type: d64::FileType::User,
        ..FileEntry::default()
    };
    disk.create_file(&entry, b"TEST");

    let entry = &disk.list_entries()[0];
    let content = disk.read_file(entry);
    println!("Read content: {:?}", content);
    Ok(())
}
