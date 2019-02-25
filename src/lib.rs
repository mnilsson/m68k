#[macro_use]
extern crate bitflags;
pub mod addressing_mode;
pub mod bus;
pub mod cpu;
pub mod decoder;
pub mod instruction_set;
pub mod mapped_hardware;
pub mod memory;
mod registers;
mod value;
pub mod vm;

fn test_bit(val: u32, bit: u32) -> bool {
    let mask = 1 << bit;
    val & mask == mask
}

fn set_bit(val: u32, bit: u32) -> u32 {
    let mask = 1 << bit;

    if val & mask == mask {
        val | mask
    } else {
        val ^ mask
    }
}
