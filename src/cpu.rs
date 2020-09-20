// Start address of BIOS
const PC_RESET_VALUE: u32 = 0xBFC00000;

pub struct Cpu {
    pc: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: PC_RESET_VALUE,
        }
    }
}