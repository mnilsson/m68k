pub trait MappedHardware {
    fn read_byte(&self, address: u32) -> Option<u8>;
    fn write_byte(&mut self, address: u32, byte: u8) -> Option<u8>;

    fn read_word(&self, address: u32) -> Option<u16> {
        let wh = self.read_byte(address);
        let wl = self.read_byte(address + 1);
        match (wh, wl) {
            (Some(h), Some(l)) => Some(((h as u16) << 8) | l as u16),
            _ => None,
        }
    }
}
