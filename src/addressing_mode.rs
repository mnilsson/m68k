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

impl Into<DataSize> for DataSizeIdentifier {
    fn into(self) -> DataSize {
        decode_data_size(self)
    }
}

fn decode_data_size(bits: DataSizeIdentifier) -> DataSize {
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
pub type Value = u32;

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
    Value(Value),
    Vector(Value),
    SR,
    CCR,
    USP,
}
impl Into<AddressingMode> for usize {
    fn into(self) -> AddressingMode {
        decode_addressing_mode(self)
    }
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

#[derive(Debug, PartialEq)]
pub enum ConditionCode {
    CC, // Carry Clear
    LS, // Lower or Same
    CS, // Carry Set
    LT, // Less Than
    EQ, // EQual
    MI, // MInus
    F,  // False (Never true)
    NE, // Not Equal
    GE, // Greater than or Equal
    PL, // Plus
    GT, // Greater Than
    T,  // True (always true)
    HI, // HIgher
    VC, // oVerflow Clear
    LE, // Less than or Equal
    VS, // oVerflow Set
}

impl Into<ConditionCode> for usize {
    fn into(self) -> ConditionCode {
        match self & 0b1111 {
            0b0000 => ConditionCode::T,
            0b0001 => ConditionCode::F,
            0b0010 => ConditionCode::HI,
            0b0011 => ConditionCode::LS,
            0b0100 => ConditionCode::CC,
            0b0101 => ConditionCode::CS,
            0b0110 => ConditionCode::NE,
            0b0111 => ConditionCode::EQ,
            0b1000 => ConditionCode::VC,
            0b1001 => ConditionCode::VS,
            0b1010 => ConditionCode::PL,
            0b1011 => ConditionCode::MI,
            0b1100 => ConditionCode::GE,
            0b1101 => ConditionCode::LT,
            0b1110 => ConditionCode::GT,
            0b1111 => ConditionCode::LE,
            _ => unreachable!(),
        }
    }
}
