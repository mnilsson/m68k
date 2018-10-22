use addressing_mode::{
    decode_addressing_mode, AddressingMode, DataSize, DataSizeIdentifier, Value,
};
use instruction_set::Instruction;

pub fn decode(opcode: usize) -> Instruction {
    let part1 = opcode >> 12;
    let part2 = (opcode >> 6) & 0b111111;
    let part3 = opcode & 0b111111;

    let part2h = (part2 & 0b111000) >> 3;
    let part2l = part2 & 0b111;

    let part3h = (part3 & 0b111000) >> 3;
    let part3l = part3 & 0b111;

    match (part1, part2, part3) {
        (0b0000, _, _) => {
            use addressing_mode::AddressingMode::Immediate;

            match part2l {
                0b100 if part3h != 0b001 => Instruction::BTST(
                    DataSize::LongWord,
                    AddressingMode::DataDirect(part2h),
                    part3.into(),
                ),
                0b101 if part3h != 0b001 => Instruction::BCHG(
                    DataSize::LongWord,
                    AddressingMode::DataDirect(part2h),
                    part3.into(),
                ),
                0b110 if part3h != 0b001 => Instruction::BCLR(
                    DataSize::LongWord,
                    AddressingMode::DataDirect(part2h),
                    part3.into(),
                ),
                0b111 if part3h != 0b001 => Instruction::BSET(
                    DataSize::LongWord,
                    AddressingMode::DataDirect(part2h),
                    part3.into(),
                ),

                0b100 | 0b101 if part3h == 0b001 => Instruction::MOVEP(
                    DataSizeIdentifier::OneBit(part2l & 0b1).into(),
                    AddressingMode::AddressIndirectDisplacement(part3l),
                    AddressingMode::DataDirect(part2h),
                ),
                0b110 | 0b111 if part3h == 0b001 => Instruction::MOVEP(
                    DataSizeIdentifier::OneBit(part2l & 0b1).into(),
                    AddressingMode::DataDirect(part2h),
                    AddressingMode::AddressIndirectDisplacement(part3l),
                ),

                _ => {
                    let size: DataSize = DataSizeIdentifier::TwoBit(part2 & 0b11).into();
                    match part2 >> 2 {
                        0b0000 => Instruction::ORI(size, Immediate, part3.into()),
                        0b0010 => Instruction::ANDI(size, Immediate, part3.into()),
                        0b0100 => Instruction::SUBI(size, Immediate, part3.into()),
                        0b0110 => Instruction::ADDI(size, Immediate, part3.into()),
                        0b1010 => Instruction::EORI(size, Immediate, part3.into()),
                        0b1100 => Instruction::CMPI(size, Immediate, part3.into()),
                        0b1000 => {
                            let addressing_mode = decode_addressing_mode(part3);
                            let size = match addressing_mode {
                                AddressingMode::DataDirect(_) => DataSize::LongWord,
                                _ => DataSize::Byte,
                            };
                            match part2 & 0b11 {
                                0b00 => Instruction::BTST(size, Immediate, addressing_mode),
                                0b01 => Instruction::BCHG(size, Immediate, addressing_mode),
                                0b10 => Instruction::BCLR(size, Immediate, addressing_mode),
                                0b11 => Instruction::BSET(size, Immediate, addressing_mode),
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        (0b0001, _, _) => {
            Instruction::MOVE(DataSize::Byte, (part2l << 3 & part2h).into(), part3.into())
        }
        (0b0010, _, _) => Instruction::MOVE(
            DataSize::LongWord,
            (part2l << 3 & part2h).into(),
            part3.into(),
        ),
        (0b0011, _, _) => {
            Instruction::MOVE(DataSize::Word, (part2l << 3 & part2h).into(), part3.into())
        }
        (0b1000, _, _) => match (part2l, part3h) {
            (0b011, _) => Instruction::DIVU(
                DataSize::Word,
                decode_addressing_mode(part3),
                AddressingMode::DataDirect(part2 >> 3),
            ),
            (0b100, 0b000) => Instruction::SBCD(
                AddressingMode::DataDirect(part2 >> 3),
                AddressingMode::DataDirect(part3 & 0b111),
            ),
            (0b100, 0b001) => Instruction::SBCD(
                AddressingMode::DataDirect(part2 >> 3),
                AddressingMode::DataDirect(part3 & 0b111),
            ),
            (0b111, _) => Instruction::DIVS(
                DataSize::Word,
                AddressingMode::DataDirect(part2 >> 3),
                decode_addressing_mode(part3),
            ),
            (_, _) => match (part2 >> 2) & 0b1 {
                0b0 => Instruction::OR(
                    DataSizeIdentifier::TwoBit(part2 & 0b11).into(),
                    decode_addressing_mode(part3),
                    AddressingMode::DataDirect(part2 >> 3),
                ),
                0b1 => Instruction::OR(
                    DataSizeIdentifier::TwoBit(part2 & 0b11).into(),
                    AddressingMode::DataDirect(part2 >> 3),
                    decode_addressing_mode(part3),
                ),
                _ => unreachable!(),
            },
        },
        (0b1001, _, _) => match (part2l, part3h) {
            (0b011, _) | (0b111, _) => Instruction::SUBA(
                DataSizeIdentifier::OneBit(part2l >> 2).into(),
                decode_addressing_mode(part3),
                AddressingMode::AddressDirect(part2h),
            ),
            (_, _) => match (part2l >> 2, part3h) {
                (0b0, 0b000) => Instruction::SUB(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    decode_addressing_mode(part3),
                    AddressingMode::DataDirect(part2h),
                ),
                (0b1, 0b000) => Instruction::SUBX(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::DataDirect(part3l),
                    AddressingMode::DataDirect(part2h),
                ),
                (0b1, 0b001) => Instruction::SUBX(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::AddressIndirectPreDecrement(part3l),
                    AddressingMode::AddressIndirectPreDecrement(part2h),
                ),
                (_, _) => Instruction::SUB(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::DataDirect(part2h),
                    decode_addressing_mode(part3),
                ),
            },
        },
        (0b1110, _, _) => {
            let size = DataSize::Word;
            let value = AddressingMode::Value(1);
            let addressing_mode = decode_addressing_mode(part3);
            match part2 {
                0b000011 => Instruction::ASRD(size, value, addressing_mode),
                0b000111 => Instruction::ASLD(size, value, addressing_mode),
                0b001011 => Instruction::LSRD(size, value, addressing_mode),
                0b001111 => Instruction::LSLD(size, value, addressing_mode),
                0b010011 => Instruction::ROXRD(size, value, addressing_mode),
                0b010111 => Instruction::ROXLD(size, value, addressing_mode),
                0b011011 => Instruction::RORD(size, value, addressing_mode),
                0b011111 => Instruction::ROLD(size, value, addressing_mode),
                _ => {
                    let size: DataSize = DataSizeIdentifier::TwoBit(part2l & 0b11).into();
                    let value = AddressingMode::Value(part2h as Value);
                    let direct = AddressingMode::DataDirect(part2h);
                    let register = AddressingMode::DataDirect(part3l);
                    match part3h {
                        0b000 if part2l & 0b100 == 0 => Instruction::ASRD(size, value, register),
                        0b000 if part2l & 0b100 != 0 => Instruction::ASLD(size, value, register),
                        0b001 if part2l & 0b100 == 0 => Instruction::LSRD(size, value, register),
                        0b001 if part2l & 0b100 != 0 => Instruction::LSLD(size, value, register),
                        0b010 if part2l & 0b100 == 0 => Instruction::ROXRD(size, value, register),
                        0b010 if part2l & 0b100 != 0 => Instruction::ROXLD(size, value, register),
                        0b011 if part2l & 0b100 == 0 => Instruction::RORD(size, value, register),
                        0b011 if part2l & 0b100 != 0 => Instruction::ROLD(size, value, register),

                        0b100 if part2l & 0b100 == 0 => Instruction::ASRD(size, direct, register),
                        0b100 if part2l & 0b100 != 0 => Instruction::ASLD(size, direct, register),
                        0b101 if part2l & 0b100 == 0 => Instruction::LSRD(size, direct, register),
                        0b101 if part2l & 0b100 != 0 => Instruction::LSLD(size, direct, register),
                        0b110 if part2l & 0b100 == 0 => Instruction::ROXRD(size, direct, register),
                        0b110 if part2l & 0b100 != 0 => Instruction::ROXLD(size, direct, register),
                        0b111 if part2l & 0b100 == 0 => Instruction::RORD(size, direct, register),
                        0b111 if part2l & 0b100 != 0 => Instruction::ROLD(size, direct, register),
                        _ => unreachable!(),
                    }
                }
            }
        }
        _ => unimplemented!("decode missing for {:04X} {:016b}", opcode, opcode),
    }
}

#[test]
fn test_decode_divu_w() {
    let opcode = 0b1000_000011_000001;
    let instruction = decode(opcode);
    assert_eq!(
        instruction,
        Instruction::DIVU(
            DataSize::Word,
            AddressingMode::DataDirect(1),
            AddressingMode::DataDirect(0),
        )
    );
}

#[test]
fn test_decode_or_z() {
    // or.z a,Dd
    //             1000 ddd0zz aaaaaa
    let opcode = 0b1000_000000_000001;
    let instruction = decode(opcode);
    assert_eq!(
        instruction,
        Instruction::OR(
            DataSize::Byte,
            AddressingMode::DataDirect(1),
            AddressingMode::DataDirect(0),
        )
    );

    // or.z Ds,a
    //             1000 sss1zz aaaaaa
    let opcode = 0b1000_001100_010000;
    let instruction = decode(opcode);
    assert_eq!(
        instruction,
        Instruction::OR(
            DataSize::Byte,
            AddressingMode::DataDirect(1),
            AddressingMode::AddressIndirect(0),
        )
    );
}

#[test]
fn test_decode_btst() {
    let opcode = 0b0000_100000_010010;
    let instruction = decode(opcode);
    assert_eq!(
        instruction,
        Instruction::BTST(
            DataSize::Byte,
            AddressingMode::Immediate,
            AddressingMode::AddressIndirect(0b010)
        )
    );

    let opcode = 0b0000_011100_000010;
    let instruction = decode(opcode);
    assert_eq!(
        instruction,
        Instruction::BTST(
            DataSize::LongWord,
            AddressingMode::DataDirect(0b011),
            AddressingMode::DataDirect(0b010)
        )
    );
}
