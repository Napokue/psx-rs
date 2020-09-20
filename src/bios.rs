use std::fs::File;
use std::path::Path;
use std::io::Read;

const MEMORY_SIZE: u32 = 512 * 1024;

pub struct Bios {
    data: [u8; MEMORY_SIZE as usize]
}

impl Bios {
    pub fn new(path: &Path) -> Self {
        let mut bios = Bios {
            data: [0; MEMORY_SIZE as usize]
        };

        let mut file = File::open(path).unwrap();
        file.read(&mut bios.data).unwrap();

        bios
    }
}