#[derive(Debug, PartialEq)]
pub enum DataSize {
    Byte,
    Word,
    LongWord,
}

pub enum DataSizeIdentifier {
    OneBit(usize),
    TwoBit(usize),
}

pub fn decode_data_size(bits: DataSizeIdentifier) -> DataSize {
    match bits {
        DataSizeIdentifier::OneBit(bits) => match bits & 0b1 {
            0b0 => DataSize::Word,
            0b1 => DataSize::LongWord,
            _ => unreachable!(),
        },
        DataSizeIdentifier::TwoBit(bits) => match bits & 0b11 {
            0b00 => DataSize::Byte,
            0b01 => DataSize::Word,
            0b10 => DataSize::LongWord,
            _ => unreachable!(),
        },
    }
}

pub type RegNr = usize;

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    DataDirect(RegNr),
    AddressDirect(RegNr),
    AddressIndirect(RegNr),
    AddressIndirectPostIncrement(RegNr),
    AddressIndirectPreDecrement(RegNr),
    AddressIndirectDisplacement(RegNr),
    AddressIndirectIndexedAndDisplacement(RegNr),
    AbsoluteAddress(DataSize),
    PCIndirectDisplacementMode,
    PCIndirectIndexed,
    Immediate,
}

pub fn decode_addressing_mode(bits: usize) -> AddressingMode {
    let part1 = bits >> 3;
    let part2 = bits & 0b111;
    match (part1, part2) {
        (0b000, _) => AddressingMode::DataDirect(part2),
        (0b001, _) => AddressingMode::AddressDirect(part2),
        (0b010, _) => AddressingMode::AddressIndirect(part2),
        (0b011, _) => AddressingMode::AddressIndirectPostIncrement(part2),
        (0b100, _) => AddressingMode::AddressIndirectPreDecrement(part2),
        (0b101, _) => AddressingMode::AddressIndirectDisplacement(part2),
        (0b110, _) => AddressingMode::AddressIndirectIndexedAndDisplacement(part2),
        (0b111, 0b000) => AddressingMode::AbsoluteAddress(DataSize::Byte),
        (0b111, 0b001) => AddressingMode::AbsoluteAddress(DataSize::Word),
        (0b111, 0b010) => AddressingMode::PCIndirectDisplacementMode,
        (0b111, 0b011) => AddressingMode::PCIndirectIndexed,
        (0b111, 0b100) => AddressingMode::Immediate,
        (_, _) => unreachable!(),
    }
}
