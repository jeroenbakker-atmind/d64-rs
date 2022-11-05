#[derive(Default)]
pub struct Sector {
    data: Vec<u8>
}

impl Sector {
    pub fn initialize(&mut self, data_len: usize) {
        self.data.clear();
        self.data.resize(data_len, 0);
    }
}