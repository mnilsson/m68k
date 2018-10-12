use addressing_mode::{decode_addressing_mode, AddressingMode, DataSize, DataSizeIdentifier};
use instruction_set::Instruction;

fn decode(opcode: usize) -> Instruction {
    let part1 = opcode >> 12;
    let part2 = (opcode >> 6) & 0b111111;
    let part3 = opcode & 0b111111;

    let part2h = (part2 & 0b111000) >> 3;
    let part2l = part2 & 0b111;

    let part3h = (part3 & 0b111000) >> 3;
    let part3l = part3 & 0b111;

    match (part1, part2, part3) {
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
            //1001 dddz11 aaaaaa                                     suba.z a,Ad
            (0b011, _) | (0b111, _) => Instruction::SUBA(
                DataSizeIdentifier::OneBit(part2l >> 2).into(),
                decode_addressing_mode(part3),
                AddressingMode::AddressDirect(part2h),
            ),
            (_, _) => match (part2l >> 2, part3h) {
                //1001 ddd0zz aaaaaa                                     sub.z   a,Dd
                (0b0, 0b000) => Instruction::SUB(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    decode_addressing_mode(part3),
                    AddressingMode::DataDirect(part2h),
                ),
                //1001 ddd1zz 000sss                                     subx.z  Ds,Dd
                (0b1, 0b000) => Instruction::SUBX(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::DataDirect(part3l),
                    AddressingMode::DataDirect(part2h),
                ),
                //1001 ddd1zz 001sss                                     subx.z  -(As),-(Ad)
                (0b1, 0b001) => Instruction::SUBX(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::AddressIndirectPreDecrement(part3l),
                    AddressingMode::AddressIndirectPreDecrement(part2h),
                ),
                //1001 sss1zz aaaaaa                                     sub.z   Ds,a
                (_, _) => Instruction::SUB(
                    DataSizeIdentifier::TwoBit(part2l & 0b11).into(),
                    AddressingMode::DataDirect(part2h),
                    decode_addressing_mode(part3),
                ),
            },
        },
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
