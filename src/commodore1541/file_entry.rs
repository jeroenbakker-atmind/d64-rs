#[derive(PartialEq, Debug)]
pub enum FileType {
    Scratched = 0x00,
    Deleted = 0x80,
    Sequence = 0x81,
    Program = 0x82,
    User = 0x83,
    Relative = 0x84,
}

impl From<u8> for FileType {
    fn from(src: u8) -> FileType {
        match src {
            0x00 => FileType::Scratched,
            0x80 => FileType::Deleted,
            0x81 => FileType::Sequence,
            0x82 => FileType::Program,
            0x83 => FileType::User,
            0x84 => FileType::Relative,
            _ => FileType::Scratched,
        }
    }
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub file_type: FileType,
    pub num_blocks: usize,
    pub start_track_no: u8,
    pub start_sector_no: u8,
}

impl FileEntry {
    pub fn from_bytes(bytes: &[u8; 16]) -> FileEntry {
        let file_type = FileType::from(bytes[2]);
        let start_track_no = bytes[3];
        let start_sector_no = bytes[4];
        FileEntry {
            name: String::new(),
            file_type,
            num_blocks: 0,
            start_track_no,
            start_sector_no,
        }
    }
}
