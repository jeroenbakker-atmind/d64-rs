use crate::{PetsciiString, Sector, SectorRef, PETSCII_NBSP};

const OFFSET_FILE_TYPE: usize = 2;
const OFFSET_START_SECTOR: usize = 3;
const OFFSET_NAME: usize = 5;
const NAME_LENGTH: usize = 16;
const OFFSET_NAME_END: usize = OFFSET_NAME + NAME_LENGTH;
const OFFSET_NUM_SECTORS: usize = 30;

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
    pub num_sectors: usize,
    pub start_sector: SectorRef,
    pub file_entry_ref: FileListEntryRef,
}

impl FileEntry {
    pub fn from_bytes(bytes: &[u8; 32], file_entry_ref: FileListEntryRef) -> FileEntry {
        let file_type = FileType::from(bytes[OFFSET_FILE_TYPE]);
        let start_sector = (bytes[OFFSET_START_SECTOR], bytes[OFFSET_START_SECTOR + 1]);
        let name = PetsciiString::fixed_size(&bytes[OFFSET_NAME..OFFSET_NAME_END]);
        let num_blocks =
            bytes[OFFSET_NUM_SECTORS + 1] as usize * 256 + bytes[OFFSET_NUM_SECTORS] as usize;

        FileEntry {
            name,
            file_type,
            num_sectors: num_blocks,
            start_sector,
            file_entry_ref,
        }
    }

    pub fn store(&self, sector: &mut Sector, offset: usize) {
        sector.set_byte(offset + OFFSET_FILE_TYPE, u8::from(self.file_type));
        sector.set_byte(offset + OFFSET_START_SECTOR, self.start_sector.0);
        sector.set_byte(offset + OFFSET_START_SECTOR + 1, self.start_sector.1);
        sector.fill(offset + OFFSET_NAME, offset + OFFSET_NAME_END, PETSCII_NBSP);
        sector.set_bytes(offset + OFFSET_NAME, self.name.as_slice());
        sector.set_byte(
            offset + OFFSET_NUM_SECTORS + 1,
            (self.num_sectors / 256) as u8,
        );
        sector.set_byte(offset + OFFSET_NUM_SECTORS, (self.num_sectors % 256) as u8)
    }

    pub fn scratch(&self, sector: &mut Sector, offset: usize) {
        sector.fill(offset + 2, offset + 32, 0);
    }
}

pub type FileListEntryRef = (SectorRef, usize);
