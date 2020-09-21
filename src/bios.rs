use std::fs::File;
use std::path::Path;
use std::io::Read;

const MEMORY_SIZE: u32 = 512 * 1024;

pub struct Bios {
    data: Vec<u8>
}

impl Bios {
    pub fn new(path: &Path) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read(&mut data).unwrap();
        Bios { data }
    }

    pub fn fetch_32(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        self.data[offset + 0] as u32 |
        (self.data[offset + 1] as u32) << 8 |
        (self.data[offset + 2] as u32) << 16 |
        (self.data[offset + 3] as u32) << 24
    }
}