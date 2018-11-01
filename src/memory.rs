use mapped_hardware::MappedHardware;

pub struct Memory {
    prg: Vec<u8>,
}

impl Memory {
    pub fn new(prg: Vec<u8>) -> Memory {
        Memory { prg: prg }
    }
}

impl MappedHardware for Memory {
    fn read_byte(&self, address: u32) -> Option<u8> {
        if self.prg.len() > address as usize {
            return Some(self.prg[address as usize]);
        }
        None
    }

    fn write_byte(&mut self, address: u32, byte: u8) -> Option<u8> {
        if self.prg.len() > address as usize {
            self.prg[address as usize] = byte;
            return Some(byte);
        }
        None
    }
}
