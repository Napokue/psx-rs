use crate::system::System;

// Start address of BIOS
const PC_RESET_VALUE: u32 = 0xBFC00000;

pub struct Cpu {
    pc: u32,
    system: System,
}

impl Cpu {
    pub fn new(system: System) -> Self {
        Cpu {
            pc: PC_RESET_VALUE,
            system,
        }
    }
}