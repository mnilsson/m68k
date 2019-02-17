use addressing_mode::{decode_addressing_mode, AddressingMode, DataSize, DataSizeIdentifier};
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
        (0b0100, 0b111001, 0b110000) => Instruction::RESET,
        (0b0100, 0b111001, 0b110001) => Instruction::NOP,
        (0b0100, 0b111001, 0b110010) => Instruction::STOP(AddressingMode::Immediate),
        (0b0100, 0b111001, 0b110011) => Instruction::RTE,
        (0b0100, 0b111001, 0b110101) => Instruction::RTS,
        (0b0100, 0b111001, 0b110110) => Instruction::TRAPV,
        (0b0100, 0b111001, 0b110111) => Instruction::RTR,

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
                    // 0000 0000zz aaaaaa kkkkkkkk kkkkkkkk kifz==lk kifz==lk ori.z   #kz,a
                    match part2 >> 2 {
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
                        _ => {
                            let size: DataSize = DataSizeIdentifier::TwoBit(part2 & 0b11).into();
                            match part2 >> 2 {
                                0b0000 => Instruction::ORI(size, Immediate, part3.into()),
                                0b0010 => Instruction::ANDI(size, Immediate, part3.into()),
                                0b0100 => Instruction::SUBI(size, Immediate, part3.into()),
                                0b0110 => Instruction::ADDI(size, Immediate, part3.into()),
                                0b1010 => Instruction::EORI(size, Immediate, part3.into()),
                                0b1100 => Instruction::CMPI(size, part3.into()),
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
        (0b0001, _, _) => {
            Instruction::MOVE(DataSize::Byte, part3.into(), (part2l << 3 | part2h).into())
        }
        (0b0010, _, _) => Instruction::MOVE(
            DataSize::LongWord,
            part3.into(),
            (part2l << 3 | part2h).into(),
        ),
        (0b0011, _, _) => {
            Instruction::MOVE(DataSize::Word, part3.into(), (part2l << 3 | part2h).into())
        }
        (0b0100, _, _) => decode_0100(opcode),
        (0b0101, _, _) => match (part2l & 0b11, part3h) {
            (0b11, 0b001) => Instruction::DB(
                (part2 >> 2).into(),
                AddressingMode::DataDirect(part3l),
                AddressingMode::Immediate,
            ),
            (0b11, _) => Instruction::ST(DataSize::Byte, (part2 >> 2).into(), part3.into()),
            (_, _) => match part2l >> 2 {
                0b0 => Instruction::ADDQ(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::Value(part2h as u32),
                    part3.into(),
                ),
                0b1 => Instruction::SUBQ(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::Value(part2h as u32),
                    part3.into(),
                ),
                _ => unreachable!(),
            },
        },
        (0b0110, _, _) => {
            // 4 highest bits of part2
            let part2h4 = (part2h << 1) | (part2l >> 2);
            let label = ((part2l & 0b11) << 6) | part3;
            let (address, size) = if label == 0 {
                (AddressingMode::Immediate, DataSize::Word)
            } else {
                (AddressingMode::Value(label as u32), DataSize::Byte)
            };
            match part2h4 {
                0b0000 => Instruction::BRA(address),
                0b0001 => Instruction::BSR(address),
                _ => Instruction::BCC(size, part2h4.into(), address),
            }
        }

        (0b0111, _, _) => Instruction::MOVEQ(
            DataSize::LongWord,
            AddressingMode::Value(((part2l << 6) | part3) as u32),
            AddressingMode::DataDirect(part2h),
        ),
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

        (0b1011, _, _) => match (part2l & 0b100, part2l & 0b11, part3h) {
            (_, 0b11, _) => Instruction::CMPA(
                DataSizeIdentifier::OneBit(part2l >> 2).into(),
                part3.into(),
                AddressingMode::AddressDirect(part2h),
            ),
            (0b000, _, _) => Instruction::CMP(
                DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                part3.into(),
                AddressingMode::DataDirect(part2h),
            ),
            (0b100, _, 0b000) => Instruction::CMPM(
                DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                AddressingMode::DataDirect(part3l),
                AddressingMode::DataDirect(part2h),
            ),
            (0b100, _, 0b001) => Instruction::CMPM(
                DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                AddressingMode::AddressIndirectPostIncrement(part3l),
                AddressingMode::AddressIndirectPostIncrement(part2h),
            ),
            (0b100, _, _) => Instruction::EOR(
                DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                part3.into(),
                AddressingMode::AddressDirect(part2h),
            ),
            (_, _, _) => unreachable!(),
        },
        (0b1100, _, _) => match (part2l, part3h) {
            (0b011, _) => Instruction::MULU(
                DataSize::Word,
                part3.into(),
                AddressingMode::DataDirect(part2h),
            ),
            (0b100, 0b000) => Instruction::ABCD(
                AddressingMode::DataDirect(part3l),
                AddressingMode::DataDirect(part2h),
            ),
            (0b101, 0b000) => Instruction::EXG(
                DataSize::LongWord,
                AddressingMode::DataDirect(part3l),
                AddressingMode::DataDirect(part2h),
            ),
            (0b100, 0b001) => Instruction::ABCD(
                AddressingMode::AddressIndirectPreDecrement(part3l),
                AddressingMode::AddressIndirectPreDecrement(part2h),
            ),
            (0b101, 0b001) => Instruction::EXG(
                DataSize::LongWord,
                AddressingMode::AddressDirect(part3l),
                AddressingMode::AddressDirect(part2h),
            ),
            (0b110, 0b001) => Instruction::EXG(
                DataSize::LongWord,
                AddressingMode::AddressDirect(part3l),
                AddressingMode::DataDirect(part2h),
            ),
            (0b111, _) => Instruction::MULS(
                DataSize::Word,
                part3.into(),
                AddressingMode::DataDirect(part2h),
            ),

            _ => match part2l >> 2 {
                0b0 => Instruction::AND(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    part3.into(),
                    AddressingMode::DataDirect(part2h),
                ),
                0b1 => Instruction::AND(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::DataDirect(part2h),
                    part3.into(),
                ),
                _ => unreachable!(),
            },
        },
        (0b1101, _, _) => {
            if part2l & 0b11 == 0b11 {
                Instruction::ADDA(
                    DataSizeIdentifier::OneBit(part2l >> 2).into(),
                    part3.into(),
                    AddressingMode::AddressDirect(part2h),
                )
            } else {
                let bit = part2l >> 2;
                match (bit, part3h) {
                    (0b1, 0b000) => Instruction::ADDX(
                        DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                        AddressingMode::DataDirect(part3l),
                        AddressingMode::DataDirect(part2h),
                    ),
                    (0b1, 0b001) => Instruction::ADDX(
                        DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                        AddressingMode::AddressIndirectPreDecrement(part3l),
                        AddressingMode::AddressIndirectPreDecrement(part2h),
                    ),
                    (0b0, _) => Instruction::ADD(
                        DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                        part3.into(),
                        AddressingMode::DataDirect(part2h),
                    ),
                    (0b1, _) => Instruction::ADD(
                        DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                        AddressingMode::DataDirect(part2h),
                        part3.into(),
                    ),

                    (_, _) => unreachable!(),
                }
            }
        }
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
                    let value = AddressingMode::Value(part2h as u32);
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

fn decode_0100(opcode: usize) -> Instruction {
    let part2 = (opcode >> 6) & 0b111111;
    let part3 = opcode & 0b111111;

    let part2h = (part2 & 0b111000) >> 3;
    let part2l = part2 & 0b111;

    let part3h = (part3 & 0b111000) >> 3;
    let part3l = part3 & 0b111;

    let one_bit_size = DataSizeIdentifier::OneBit(part2l & 0b1);
    let two_bit_size = DataSizeIdentifier::TwoBit(part2l & 0b11);
    match part2l >> 2 {
        0b0 => match (part2h, part2l, part3h) {
            (0b000, 0b011, _) => {
                Instruction::MOVE(DataSize::Word, AddressingMode::SR, part3.into())
            }
            (0b000, _, _) => Instruction::NEGX(two_bit_size.into(), part3.into()),
            (0b001, _, _) => Instruction::CLR(two_bit_size.into(), part3.into()),
            (0b010, 0b011, _) => {
                Instruction::MOVE(DataSize::Word, part3.into(), AddressingMode::CCR)
            }
            (0b010, _, _) => Instruction::NEG(two_bit_size.into(), part3.into()),
            (0b011, 0b011, _) => {
                Instruction::MOVE(DataSize::Word, part3.into(), AddressingMode::SR)
            }
            (0b011, _, _) => Instruction::NOT(two_bit_size.into(), part3.into()),

            (0b100, 0b000, _) => Instruction::NBCD(part3.into()),
            (0b100, 0b001, 0b000) => {
                Instruction::SWAP(DataSize::Word, AddressingMode::DataDirect(part3l))
            }
            (0b100, 0b001, _) => Instruction::PEA(part3.into()),
            (0b100, _, 0b000) => {
                Instruction::EXT(one_bit_size.into(), AddressingMode::DataDirect(part3l))
            }
            (0b100, _, _) => Instruction::MOVEM(one_bit_size.into(), part3.into(), 0),
            (0b101, 0b011, _) => Instruction::TAS(DataSize::Byte, part3.into()),
            (0b101, _, _) => Instruction::TST(two_bit_size.into(), part3.into()),
            (0b110, _, _) => Instruction::MOVEM(one_bit_size.into(), part3.into(), 1),
            (0b111, 0b001, 0b010) => Instruction::LINK(
                AddressingMode::AddressDirect(part3l),
                AddressingMode::Immediate,
            ),
            (0b111, 0b001, 0b011) => Instruction::UNLK(AddressingMode::AddressDirect(part3l)),
            (0b111, 0b001, 0b100) => Instruction::MOVE(
                DataSize::Word,
                AddressingMode::AddressDirect(part3l),
                AddressingMode::USP,
            ),
            (0b111, 0b001, 0b101) => Instruction::MOVE(
                DataSize::Word,
                AddressingMode::USP,
                AddressingMode::AddressDirect(part3l),
            ),
            (0b111, 0b001, _) => Instruction::TRAP(AddressingMode::Vector(part3 as u32)),
            (0b111, 0b010, _) => Instruction::JSR(part3.into()),
            (0b111, 0b011, _) => Instruction::JMP(part3.into()),
            _ => unreachable!(),
        },
        0b1 => match part2l {
            0b110 => Instruction::CHK(
                DataSize::Word,
                part3.into(),
                AddressingMode::DataDirect(part2h),
            ),
            0b111 => Instruction::LEA(part3.into(), AddressingMode::AddressDirect(part2h)),
            _ => panic!("{:04X} {:016b}", opcode, opcode),
        },
        _ => unreachable!(),
    }
}
