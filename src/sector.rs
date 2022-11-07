use std::io::{Read, Write};

use crate::decode_petscii;

#[derive(Default)]
pub struct Sector {
    data: Vec<u8>,
}

impl Sector {
    pub fn initialize(&mut self, data_len: usize) {
        self.data.clear();
        self.data.resize(data_len, 0);
    }

    pub fn read_from_reader<R: Read>(&mut self, reader: &mut R) -> std::io::Result<usize> {
        // TODO raise error when not all bytes could be read.
        reader.read(self.data.as_mut_slice())
    }
    pub fn write_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
        writer.write(self.data.as_slice())
    }

    pub fn get_byte(&self, offset: usize) -> &u8 {
        &self.data[offset]
    }
    pub fn set_byte(&mut self, offset: usize, byte: u8) {
        self.data[offset] = byte;
    }
    pub fn get_bytes(&self, offset: usize, result: &mut [u8]) {
        for (i, item) in result.iter_mut().enumerate() {
            let byte = *self.get_byte(offset + i);
            *item = byte;
        }
    }
    pub fn set_bytes(&mut self, offset: usize, result: &[u8]) {
        for (i, item) in result.iter().enumerate() {
            self.set_byte(offset + i, *item);
        }
    }
    pub fn fill(&mut self, start_offset: usize, end_offset: usize, byte: u8) {
        for offset in start_offset..end_offset {
            self.set_byte(offset, byte);
        }
    }

    pub fn print(&self) {
        let mut x = 0;
        let mut decoded = String::new();
        for a in &self.data {
            print!("{:02x} ", a);
            decoded.push(decode_petscii(*a) as char);
            x += 1;
            if x == 16 {
                x = 0;
                println!("  {}", decoded);
                decoded.clear();
            }
        }
    }
}
