use std::io::Read;

#[derive(Default)]
pub struct Sector {
    data: Vec<u8>,
}

impl Sector {
    pub fn initialize(&mut self, data_len: usize) {
        self.data.clear();
        self.data.resize(data_len, 0);
    }

    pub fn read_from_reader<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        // TODO raise error when not all bytes could be read.
        reader.read(self.data.as_mut_slice())?;
        Ok(())
    }

    pub fn print(&self) {
        let mut x = 0;
        for a in &self.data {
            print!("{:02x} ", a);
            x += 1;
            if x == 16 {
                x = 0;
                println!();
            }
        }
    }
}
