mod cpu;
mod bios;

use cpu::Cpu;
use bios::Bios;
use std::fs::File;
use std::path::Path;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let bios = Bios::new(&Path::new(&args[0]));
    let cpu = Cpu::new();
}
