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

    pub fn next_instruction(&mut self) {
        let pc = self.pc;
        let instruction = self.load_32(pc);
        self.pc = pc.wrapping_add(4);
        self.decode_and_execute(instruction);
    }

    fn load_32(&self, address: u32) -> u32 {
        self.system.load_32(address)
    }

    fn decode_and_execute(&mut self, instruction: u32) {

    }
}