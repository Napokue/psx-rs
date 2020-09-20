use crate::bios::Bios;

pub struct System {
    bios: Bios
}

impl System {
    pub fn new(bios: Bios) -> Self {
        System {
            bios
        }
    }
}