use mapped_hardware::MappedHardware;

pub struct Memory {
    prg: Vec<u8>,
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(prg: Vec<u8>) -> Memory {
        let memory = Memory {
            prg: prg,
            memory: vec![0; 0xffff_ffff],
        };
        memory
    }
}

impl MappedHardware for Memory {
    fn read_word(&mut self, address: u32) -> Option<u16> {
        if self.prg.len() > address as usize {
            let hbyte = (self.prg[address as usize] as u16) << 8;
            let lbyte = self.prg[(address + 1) as usize] as u16;
            return Some(hbyte | lbyte);
        } else {
            let hbyte = (self.memory[address as usize] as u16) << 8;
            let lbyte = self.memory[(address + 1) as usize] as u16;
            return Some(hbyte | lbyte);
        }
    }

    fn write_word(&mut self, address: u32, value: u16) -> Option<u16> {
        let hbyte = (value >> 8) as u8;
        let lbyte = value as u8;
        if self.prg.len() > (address + 1) as usize {
            self.prg[address as usize] = hbyte;
            self.prg[(address + 1) as usize] = lbyte;
            println!("prgwr");
            Some(value)
        } else {
            self.memory[address as usize] = hbyte;
            self.memory[(address + 1) as usize] = lbyte;
            println!("memwr {:?} {:?}", address, value);
            Some(value)
        }
    }
}
