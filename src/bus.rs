use mapped_hardware::MappedHardware;

#[derive(Default)]
pub struct Bus {
    mapped_hardwares: Vec<Box<MappedHardware>>,
}

impl Bus {
    pub fn map_hardware(&mut self, hardware: Box<MappedHardware>) {
        self.mapped_hardwares.push(hardware);
    }
}

impl MappedHardware for Bus {
    fn read_byte(&self, address: u32) -> Option<u8> {
        for mut hw in &self.mapped_hardwares {
            if let Some(ref byte) = hw.read_byte(address) {
                return Some(*byte);
            }
        }
        None
    }

    fn write_byte(&mut self, address: u32, byte: u8) -> Option<u8> {
        for mut hw in &mut self.mapped_hardwares {
            if let Some(ref byte) = hw.write_byte(address, byte) {
                return Some(*byte);
            }
        }
        None
    }
}
