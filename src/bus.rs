use mapped_hardware::MappedHardware;

#[derive(Default)]
pub struct Bus {
    mapped_hardwares: Vec<Box<MappedHardware>>,
    pub cycles: u64,
}

impl Bus {
    pub fn map_hardware(&mut self, hardware: Box<MappedHardware>) {
        self.mapped_hardwares.push(hardware);
    }
}

impl MappedHardware for Bus {
    fn tick(&mut self, cycles: usize) {
        self.cycles += cycles as u64;
        for mut hw in self.mapped_hardwares.iter_mut() {
            hw.tick(cycles);
        }
    }

    fn read_word(&mut self, address: u32) -> Option<u16> {
        self.tick(4);
        for mut hw in &mut self.mapped_hardwares {
            if let Some(ref byte) = hw.read_word(address) {
                return Some(*byte);
            }
        }
        None
    }

    fn write_word(&mut self, address: u32, value: u16) -> Option<u16> {
        self.tick(4);
        for mut hw in &mut self.mapped_hardwares {
            if let Some(ref byte) = hw.write_word(address, value) {
                return Some(*byte);
            }
        }
        None
    }
}
