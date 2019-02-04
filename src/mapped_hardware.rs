pub trait MappedHardware {
    fn tick(&mut self, cycles: usize) {}

    fn read_byte(&mut self, address: u32) -> Option<u8> {
        if let Some(byte) = self.read_word(address) {
            Some(byte as u8)
        } else {
            None
        }
    }

    fn read_word(&mut self, address: u32) -> Option<u16>;

    fn read_long(&mut self, address: u32) -> Option<u32> {
        let wh = self.read_word(address);
        let wl = self.read_word(address + 2);
        match (wh, wl) {
            (Some(h), Some(l)) => Some(((h as u32) << 16) | l as u32),
            _ => None,
        }
    }
    fn write_word(&mut self, address: u32, value: u16) -> Option<u16>;

    fn write_long(&mut self, address: u32, value: u32) -> Option<u32> {
        let wh = (value >> 16) as u16;
        let wl = value as u16;
        let writeh = self.write_word(address, wh);
        if let None = writeh {
            return None;
        };
        let writel = self.write_word(address + 2, wl);
        if let None = writel {
            return None;
        };
        Some(value)
    }
}
