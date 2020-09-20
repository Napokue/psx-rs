mod cpu;
mod bios;
mod system;

use cpu::Cpu;
use bios::Bios;
use system::System;

use std::fs::File;
use std::path::Path;
use std::env;


fn main() {
    let args = env::args().collect::<Vec<_>>();
    let bios = Bios::new(&Path::new(&args[0]));

    let system = System::new(bios);

    let cpu = Cpu::new(system);
}
