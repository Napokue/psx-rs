use crate::bios::Bios;
use crate::map;

pub struct System {
    bios: Bios
}

impl System {
    pub fn new(bios: Bios) -> Self {
        System {
            bios
        }
    }

    pub fn load_32(&self, address: u32) -> u32 {
        if let Some(offset) = map::BIOS.contains(address) {
            self.bios.fetch_32(offset)
        } else {
            panic!("Address could not be fetched")
        }
    }
}