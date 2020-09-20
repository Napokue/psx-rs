pub const BIOS: Range = Range(0xbfc00000 , 512 * 1024);

pub struct Range(u32, u32);

impl Range {
    pub fn contains(self, address: u32) -> Option<u32> {
        let Range(start,length) = self;

        if address >= start && address <= start + length {
            Some(address - start)
        } else {
            None
        }
    }
}