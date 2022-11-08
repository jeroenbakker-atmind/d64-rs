use crate::{PetsciiString, Sector, SectorRef, PETSCII_NBSP};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

impl From<FileType> for u8 {
    fn from(src: FileType) -> u8 {
        src as u8
    }
}

impl Default for FileType {
    fn default() -> FileType {
        FileType::Program
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileEntry {
    pub name: PetsciiString,
    pub file_type: FileType,
    pub num_blocks: usize,
    pub start_sector: SectorRef,
    pub file_entry_ref: FileListEntryRef,
}

impl FileEntry {
    pub fn from_bytes(bytes: &[u8; 32], file_entry_ref: FileListEntryRef) -> FileEntry {
        let file_type = FileType::from(bytes[2]);
        let start_sector = (bytes[3], bytes[4]);
        let name = PetsciiString::fixed_size(&bytes[5..21]);
        let num_blocks = bytes[31] as usize * 256 + bytes[30] as usize;
        FileEntry {
            name,
            file_type,
            num_blocks,
            start_sector,
            file_entry_ref,
        }
    }

    pub fn store(&self, sector: &mut Sector, offset: usize) {
        sector.set_byte(offset + 2, u8::from(self.file_type));
        sector.set_byte(offset + 3, self.start_sector.0);
        sector.set_byte(offset + 4, self.start_sector.1);
        sector.fill(offset + 5, offset + 5 + 16, PETSCII_NBSP);
        sector.set_bytes(offset + 5, self.name.bytes.as_slice());
        sector.set_byte(offset + 31, (self.num_blocks / 256) as u8);
        sector.set_byte(offset + 30, (self.num_blocks % 256) as u8)
    }
    pub fn scratch(&self, sector: &mut Sector, offset: usize) {
        sector.fill(offset + 2, offset + 32, 0);
    }
}

pub type FileListEntryRef = (SectorRef, usize);
