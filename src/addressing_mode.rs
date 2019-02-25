use crate::test_bit;
use cpu::Cpu;
use mapped_hardware::MappedHardware;
use registers::ConditionCode;
use value::Value;
#[derive(Debug, PartialEq, Copy, Clone)]
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
    Value(u32),
    Vector(u32),
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
        (0b111, 0b000) => AddressingMode::AbsoluteAddress(DataSize::Word),
        (0b111, 0b001) => AddressingMode::AbsoluteAddress(DataSize::LongWord),
        (0b111, 0b010) => AddressingMode::PCIndirectDisplacementMode,
        (0b111, 0b011) => AddressingMode::PCIndirectIndexed,
        (0b111, 0b100) => AddressingMode::Immediate,
        (_, _) => unreachable!(),
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
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

impl Into<Condition> for usize {
    fn into(self) -> Condition {
        match self & 0b1111 {
            0b0000 => Condition::T,
            0b0001 => Condition::F,
            0b0010 => Condition::HI,
            0b0011 => Condition::LS,
            0b0100 => Condition::CC,
            0b0101 => Condition::CS,
            0b0110 => Condition::NE,
            0b0111 => Condition::EQ,
            0b1000 => Condition::VC,
            0b1001 => Condition::VS,
            0b1010 => Condition::PL,
            0b1011 => Condition::MI,
            0b1100 => Condition::GE,
            0b1101 => Condition::LT,
            0b1110 => Condition::GT,
            0b1111 => Condition::LE,
            _ => unreachable!(),
        }
    }
}

pub fn read_addressing_mode_address(
    cpu: &mut Cpu,
    bus: &mut impl MappedHardware,
    size: &DataSize,
    addressing_mode: &AddressingMode,
) -> u32 {
    fn map_value(size: &DataSize, value: u32) -> Value {
        match size {
            DataSize::Byte => Value::Byte(value as u8),
            DataSize::Word => Value::Word(value as u16),
            DataSize::LongWord => Value::LongWord(value),
        }
    };

    match addressing_mode {
        AddressingMode::Immediate => cpu.read_immediate(bus, size).into(),
        AddressingMode::AbsoluteAddress(DataSize::Word) => {
            let addr = cpu.read_immediate(bus, &DataSize::Word).into();
            addr
        }
        AddressingMode::AbsoluteAddress(DataSize::LongWord) => {
            let addr = cpu.read_immediate(bus, &DataSize::LongWord).into();
            addr
        }
        AddressingMode::AddressIndirect(ref reg) => {
            let addr = cpu.registers.address(*reg);
            addr
        }
        AddressingMode::AddressIndirectPostIncrement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            let incr = match size {
                DataSize::Byte => {
                    if *reg == 7 {
                        2
                    } else {
                        1
                    }
                }
                DataSize::Word => 2,
                DataSize::LongWord => 4,
            };

            cpu.registers.set_address(*reg, addr + incr);
            addr
        }
        AddressingMode::AddressIndirectPreDecrement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            let addr = match size {
                DataSize::Byte => {
                    if *reg == 7 {
                        addr.wrapping_sub(2)
                    } else {
                        addr.wrapping_sub(1)
                    }
                }
                DataSize::Word => addr.wrapping_sub(2),
                DataSize::LongWord => addr.wrapping_sub(4),
            };
            cpu.registers.set_address(*reg, addr);
            addr
        }
        AddressingMode::AddressIndirectDisplacement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            let displacement: u16 = cpu.read_immediate(bus, &DataSize::Word).into();
            let addr = (addr as i64 - displacement as u16 as i64) as u32;

            addr
        }
        AddressingMode::AddressIndirectIndexedAndDisplacement(ref reg) => {
            let reg_addr = cpu.registers.address(*reg);

            let extension_word: u16 = cpu.read_immediate(bus, &DataSize::Word).into();
            let reg = extension_word >> 12 & 0b111;
            let index_value = match extension_word & 0x8000 {
                0b0 => cpu.registers.data(reg as usize),
                _ => cpu.registers.address(reg as usize),
                // _ => unreachable!("Not implemented"),
            };

            let index_size = match test_bit(extension_word.into(), 11) {
                true => DataSize::Word,
                false => DataSize::LongWord,
            };

            let scale = match (extension_word >> 8) & 0b11 {
                0b00 => 1,
                0b01 => 2,
                0b10 => 4,
                0b11 => 8,
                _ => unreachable!(),
            };

            if test_bit(extension_word.into(), 8) {
                let base_register_supress = test_bit(extension_word.into(), 7);
                let index_supress = test_bit(extension_word.into(), 6);

                // 00 => reserved
                // 01 => null displacement
                // 10 => word
                // 11 => long
                let base_displacement_size = (extension_word >> 4) & 0b11;

                match (index_supress, extension_word & 0b111) {
                    (_, 0b000) => (), // no memory indirect action,
                    (_, _) => unreachable!("check section 2.2 in 68kpm"),
                }

                unreachable!()
            } else {
                let displacement = (extension_word & 0xff) as u32;

                reg_addr + displacement + index_value * scale
            }
        }
        AddressingMode::PCIndirectDisplacementMode => {
            let pc = cpu.registers.pc();
            let ext: u32 = cpu.read_immediate(bus, &DataSize::Word).into();

            let displacement = if ext & 0x80 == 0x80 {
                ext | 0xffff_ff00
            } else {
                ext & 0xff
            };

            let idxreg = ((ext >> 12) & 0x07) as usize;
            let idxsize = if ext & 0x800 == 0x800 {
                DataSize::LongWord
            } else {
                DataSize::Word
            };
            let idx_is_addr = ext & 0x8000 == 0x8000;

            let idx_val = match (idx_is_addr, idxsize) {
                (true, DataSize::Word) => cpu.registers.address(idxreg) as u16 as u32,
                (true, DataSize::LongWord) => cpu.registers.address(idxreg),
                (false, DataSize::Word) => cpu.registers.data(idxreg) as u16 as u32,
                (false, DataSize::LongWord) => cpu.registers.data(idxreg),
                _ => unreachable!(),
            };
            pc + displacement + idx_val
        }
        _ => unreachable!("{:?}", addressing_mode),
    }
}

pub fn read_addressing_mode(
    cpu: &mut Cpu,
    bus: &mut impl MappedHardware,
    size: &DataSize,
    addressing_mode: &AddressingMode,
) -> Value {
    fn map_value(size: &DataSize, value: u32) -> Value {
        match size {
            DataSize::Byte => Value::Byte(value as u8),
            DataSize::Word => Value::Word(value as u16),
            DataSize::LongWord => Value::LongWord(value),
        }
    };

    match addressing_mode {
        AddressingMode::Value(ref value) => map_value(size, *value),
        AddressingMode::DataDirect(ref reg) => map_value(size, cpu.registers.data(*reg)),
        AddressingMode::AddressDirect(ref reg) => map_value(size, cpu.registers.address(*reg)),
        AddressingMode::Immediate => cpu.read_immediate(bus, size).into(),
        AddressingMode::AbsoluteAddress(DataSize::Word) => {
            let addr = cpu.read_immediate(bus, &DataSize::Word).into();
            bus.read_word(addr).into()
        }
        AddressingMode::AbsoluteAddress(DataSize::LongWord) => {
            let addr = cpu.read_immediate(bus, &DataSize::LongWord).into();
            let v = match size {
                DataSize::Byte => bus.read_word(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            };
            v
        }
        AddressingMode::AddressIndirect(ref reg) => {
            let addr = cpu.registers.address(*reg);
            match size {
                DataSize::Byte => bus.read_byte(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            }
        }
        AddressingMode::AddressIndirectPostIncrement(ref reg) => {
            let addr = read_addressing_mode_address(cpu, bus, size, addressing_mode);
            match size {
                DataSize::Byte => bus.read_byte(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            }
        }
        AddressingMode::AddressIndirectPreDecrement(ref reg) => {
            let addr = read_addressing_mode_address(cpu, bus, size, addressing_mode);
            match size {
                DataSize::Byte => bus.read_byte(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            }
        }
        AddressingMode::AddressIndirectDisplacement(ref reg) => {
            let addr = read_addressing_mode_address(cpu, bus, size, addressing_mode);
            match size {
                DataSize::Byte => bus.read_byte(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            }
        }
        AddressingMode::AddressIndirectIndexedAndDisplacement(ref reg) => {
            let addr = read_addressing_mode_address(cpu, bus, size, addressing_mode);
            match size {
                DataSize::Byte => bus.read_byte(addr).into(),
                DataSize::Word => bus.read_word(addr).into(),
                DataSize::LongWord => bus.read_long(addr).into(),
            }
        }
        AddressingMode::PCIndirectDisplacementMode => {
            let addr = read_addressing_mode_address(cpu, bus, size, addressing_mode);
            Value::LongWord(addr)
        }
        AddressingMode::SR => {
            let sr = cpu.registers.sr();
            Value::Word(sr)
        }
        AddressingMode::CCR => Value::Word(cpu.registers.ccr.bits().into()),
        _ => unreachable!("{:?}", addressing_mode),
    }
}

pub fn write_addressing_mode(
    cpu: &mut Cpu,
    bus: &mut impl MappedHardware,
    size: DataSize,
    addressing_mode: AddressingMode,
    value: Value,
) {
    match addressing_mode {
        AddressingMode::DataDirect(reg) => {
            let current: u32 = cpu.registers.data(reg);
            let current = match size {
                DataSize::Byte => current & 0xffff_ff00,
                DataSize::Word => current & 0xffff_0000,
                DataSize::LongWord => 0,
            };
            let value: u32 = value.into();
            cpu.registers.set_data(reg, current | value)
        }
        AddressingMode::AddressDirect(reg) => cpu.registers.set_address(reg, value.into()),
        AddressingMode::AddressIndirect(reg) => {
            match size {
                DataSize::Byte => {
                    bus.write_byte(cpu.registers.address(reg), value.into());
                }
                DataSize::Word => {
                    bus.write_word(cpu.registers.address(reg), value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(cpu.registers.address(reg), value.into());
                }
            };
        }
        AddressingMode::AddressIndirectPreDecrement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            let addr = match size {
                DataSize::Byte => addr.wrapping_sub(1),
                DataSize::Word => addr.wrapping_sub(2),
                DataSize::LongWord => addr.wrapping_sub(4),
            };
            cpu.registers.set_address(*reg, addr);
            match size {
                DataSize::Byte => {
                    bus.write_byte(addr, value.into());
                }
                DataSize::Word => {
                    bus.write_word(addr, value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(addr, value.into());
                }
            };
        }
        AddressingMode::AddressIndirectPostIncrement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            match size {
                DataSize::Byte => {
                    bus.write_byte(addr, value.into());
                }
                DataSize::Word => {
                    bus.write_word(addr, value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(addr, value.into());
                }
            };
            let addr = match size {
                DataSize::Byte => addr.wrapping_add(1),
                DataSize::Word => addr.wrapping_add(2),
                DataSize::LongWord => addr.wrapping_add(4),
            };
            cpu.registers.set_address(*reg, addr);
        }
        AddressingMode::AddressIndirectDisplacement(ref reg) => {
            let addr = cpu.registers.address(*reg);
            let displacement: u16 = cpu.read_immediate(bus, &DataSize::Word).into();
            let addr = (addr as i64 - displacement as u16 as i64) as u32;

            match size {
                DataSize::Byte => {
                    bus.write_byte(addr, value.into());
                }
                DataSize::Word => {
                    bus.write_word(addr, value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(addr, value.into());
                }
            }
        }
        AddressingMode::AddressIndirectIndexedAndDisplacement(ref reg) => {
            let addr = read_addressing_mode_address(cpu, bus, &size, &addressing_mode);
            match size {
                DataSize::Byte => {
                    bus.write_byte(addr, value.into());
                }
                DataSize::Word => {
                    bus.write_word(addr, value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(addr, value.into());
                }
            }
        }
        AddressingMode::SR => cpu.registers.set_ccr(value.into()),
        AddressingMode::USP => cpu.registers.set_usp(value.into()),
        AddressingMode::AbsoluteAddress(addr_size) => {
            let address = cpu.read_immediate(bus, &addr_size).into();
            // let a: u32 = address;
            match size {
                DataSize::Byte => {
                    bus.write_byte(address, value.into());
                }
                DataSize::Word => {
                    bus.write_word(address, value.into());
                }
                DataSize::LongWord => {
                    bus.write_long(address, value.into());
                }
            };
        }
        AddressingMode::CCR => {
            // let value: u8 = value.into();
            cpu.registers.ccr = ConditionCode::from_bits_truncate(value.into());
        }
        _ => unimplemented!("{:?}", addressing_mode),
    };
}
