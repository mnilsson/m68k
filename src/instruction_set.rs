use addressing_mode::{AddressingMode, DataSize};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    DIVU(DataSize, AddressingMode, AddressingMode),
    DIVS(DataSize, AddressingMode, AddressingMode),
    OR(DataSize, AddressingMode, AddressingMode),
    SBCD(AddressingMode, AddressingMode),
    SUB(DataSize, AddressingMode, AddressingMode),
    SUBA(DataSize, AddressingMode, AddressingMode),
    SUBX(DataSize, AddressingMode, AddressingMode),
}
