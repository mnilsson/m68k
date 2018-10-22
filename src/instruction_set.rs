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
    ASRD(DataSize, AddressingMode, AddressingMode),
    ASLD(DataSize, AddressingMode, AddressingMode),
    LSRD(DataSize, AddressingMode, AddressingMode),
    LSLD(DataSize, AddressingMode, AddressingMode),
    ROXRD(DataSize, AddressingMode, AddressingMode),
    ROXLD(DataSize, AddressingMode, AddressingMode),
    RORD(DataSize, AddressingMode, AddressingMode),
    ROLD(DataSize, AddressingMode, AddressingMode),

    ORI(DataSize, AddressingMode, AddressingMode),
    ANDI(DataSize, AddressingMode, AddressingMode),
    SUBI(DataSize, AddressingMode, AddressingMode),
    ADDI(DataSize, AddressingMode, AddressingMode),
    EORI(DataSize, AddressingMode, AddressingMode),
    CMPI(DataSize, AddressingMode, AddressingMode),

    BTST(DataSize, AddressingMode, AddressingMode),
    BCHG(DataSize, AddressingMode, AddressingMode),
    BCLR(DataSize, AddressingMode, AddressingMode),
    BSET(DataSize, AddressingMode, AddressingMode),
    MOVEP(DataSize, AddressingMode, AddressingMode),
    MOVE(DataSize, AddressingMode, AddressingMode),
}
