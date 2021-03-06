use addressing_mode::{AddressingMode, Condition, DataSize};

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
    CMPI(DataSize, AddressingMode),

    BTST(DataSize, AddressingMode, AddressingMode),
    BCHG(DataSize, AddressingMode, AddressingMode),
    BCLR(DataSize, AddressingMode, AddressingMode),
    BSET(DataSize, AddressingMode, AddressingMode),
    MOVEP(DataSize, AddressingMode, AddressingMode),
    MOVE(DataSize, AddressingMode, AddressingMode),
    ADDA(DataSize, AddressingMode, AddressingMode),
    ADDX(DataSize, AddressingMode, AddressingMode),
    ADD(DataSize, AddressingMode, AddressingMode),
    RESET,
    NOP,
    STOP(AddressingMode),
    RTE,
    RTS,
    TRAPV,
    RTR,

    AND(DataSize, AddressingMode, AddressingMode),
    MULU(DataSize, AddressingMode, AddressingMode),
    ABCD(AddressingMode, AddressingMode),
    EXG(DataSize, AddressingMode, AddressingMode),
    MULS(DataSize, AddressingMode, AddressingMode),

    BRA(AddressingMode),
    BSR(AddressingMode),
    BCC(DataSize, Condition, AddressingMode),
    MOVEQ(DataSize, AddressingMode, AddressingMode),

    CMP(DataSize, AddressingMode, AddressingMode),
    CMPA(DataSize, AddressingMode, AddressingMode),
    CMPM(DataSize, AddressingMode, AddressingMode),
    EOR(DataSize, AddressingMode, AddressingMode),

    SUBQ(DataSize, AddressingMode, AddressingMode),
    ADDQ(DataSize, AddressingMode, AddressingMode),
    ST(DataSize, Condition, AddressingMode),
    DB(Condition, AddressingMode, AddressingMode),

    LEA(AddressingMode, AddressingMode),
    CHK(DataSize, AddressingMode, AddressingMode),
    JMP(AddressingMode),
    JSR(AddressingMode),
    TRAP(AddressingMode),
    UNLK(AddressingMode),
    LINK(AddressingMode, AddressingMode),
    MOVEM(DataSize, AddressingMode, u8),
    TST(DataSize, AddressingMode),
    TAS(DataSize, AddressingMode),
    EXT(DataSize, AddressingMode),
    PEA(AddressingMode),
    SWAP(DataSize, AddressingMode),
    NBCD(AddressingMode),
    NOT(DataSize, AddressingMode),
    NEG(DataSize, AddressingMode),
    NEGX(DataSize, AddressingMode),
    CLR(DataSize, AddressingMode),
}
