use addressing_mode::{
    decode_addressing_mode, decode_data_size, AddressingMode, DataSize, DataSizeIdentifier,
};
use instruction_set::Instruction;

fn decode(opcode: usize) -> Instruction {
    let part1 = opcode >> 12;
    let part2 = (opcode >> 6) & 0b111111;
    let part3 = opcode & 0b111111;

    match (part1, part2, part3) {
        (0b1000, _, _) => match (part2 & 0b111, (part3 & 0b111000) >> 3) {
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
            (_, _) => match (part2 >> 2) & 0b000100 {
                0b0 => Instruction::OR(
                    decode_data_size(DataSizeIdentifier::TwoBit(part2 & 0b11)),
                    decode_addressing_mode(part3),
                    AddressingMode::DataDirect(part2 >> 3),
                ),
                0b1 => Instruction::OR(
                    decode_data_size(DataSizeIdentifier::TwoBit(part2 & 0b11)),
                    AddressingMode::DataDirect(part2 >> 3),
                    decode_addressing_mode(part3),
                ),
                _ => unreachable!(),
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
            AddressingMode::AddressIndirect(0),
            AddressingMode::DataDirect(1),
        )
    );
}
