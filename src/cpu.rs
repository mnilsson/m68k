use addressing_mode::{AddressingMode, Condition, DataSize};
use decoder::decode;
use instruction_set::Instruction;
use mapped_hardware::MappedHardware;
use registers::{ConditionCode, Registers};

use std::ops::Add;

#[derive(Debug)]
enum InstructionStep {
    InstructionRead,
    Instruction,
    InstructionNext,
}

impl Default for InstructionStep {
    fn default() -> Self {
        InstructionStep::InstructionNext
    }
}

struct Address(u32);

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Byte(u8),
    Word(u16),
    LongWord(u32),
}

impl Value {
    fn from_raw(size: DataSize, value: u32) -> Value {
        match size {
            DataSize::Byte => Value::Byte(value as u8),
            DataSize::Word => Value::Word(value as u16),
            DataSize::LongWord => Value::LongWord(value as u32),
        }
    }
    fn or_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();

                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s | v;

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
        }
    }

    fn add_cc(self, size: DataSize, value: Value) -> (Value, ConditionCode) {
        let mut cc: ConditionCode = ConditionCode::empty();
        match size {
            DataSize::Byte => {
                let s: u8 = self.into();
                let v: u8 = value.into();
                let r = s as u16 + v as u16;
                let max = 0xff;
                let neg = 0x80;
                if r > 0xff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x80 == 0x80 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Byte(r as u8), cc)
            }
            DataSize::Word => {
                let s: u16 = self.into();
                let v: u16 = value.into();
                let r = s as u32 + v as u32;

                if r > 0xffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000 == 0x8000 {
                    cc.set(ConditionCode::N, true);
                }

                (Value::Word(r as u16), cc)
            }
            DataSize::LongWord => {
                let s: u32 = self.into();
                let v: u32 = value.into();
                let r = s as u64 + v as u64;

                if r > 0xffff_ffff {
                    cc.set(ConditionCode::C, true);
                    cc.set(ConditionCode::X, true);
                    cc.set(ConditionCode::V, true);
                }

                if r == 0 {
                    cc.set(ConditionCode::Z, true);
                }

                if r & 0x8000_0000 == 0x8000_0000 {
                    cc.set(ConditionCode::N, true);
                }

                println!("{}", r);
                (Value::LongWord(r as u32), cc)
            }
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        match self {
            Value::Byte(v) => v as i8 as i32,
            Value::Word(v) => v as i16 as i32,
            Value::LongWord(v) => v as i32,
        }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::Byte(v) => v as u32,
            Value::Word(v) => v as u32,
            Value::LongWord(v) => v as u32,
        }
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::Byte(v) => v as u8,
            Value::Word(v) => v as u8,
            Value::LongWord(v) => v as u8,
        }
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::Byte(v) => v as u16,
            Value::Word(v) => v as u16,
            Value::LongWord(v) => v as u16,
        }
    }
}

impl Into<Value> for Option<u32> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::LongWord(val),
            _ => panic!("Not valid"),
        }
    }
}

impl Into<Value> for Option<u16> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::Word(val),
            _ => panic!("Not valid"),
        }
    }
}
impl Into<Value> for Option<u8> {
    fn into(self) -> Value {
        match self {
            Some(val) => Value::Byte(val),
            _ => panic!("Not valid"),
        }
    }
}
const ADDRESS_BUS_MASK: u32 = 0x7f_ffff;

// directions
// in = into cpu
// out = out from cpu

#[derive(Default, Debug)]
pub struct Cpu {
    registers: Registers,
    instruction_step: InstructionStep,
    instruction_clock: usize,

    immediate: Option<Value>,
}

impl Cpu {
    pub fn tick<M: MappedHardware>(&mut self, bus: &M) {}

    pub fn set_pc(&mut self, new_pc: u32) {
        self.registers.set_pc(new_pc);
    }

    pub fn set_sp(&mut self, new_pc: u32) {
        self.registers.set_sp(new_pc);
    }

    pub fn execute_next_instruction(&mut self, bus: &mut impl MappedHardware) {
        self.immediate = None;
        let op = bus.read_word(self.registers.pc()).unwrap();
        self.registers.pc_increment();
        self.registers.pc_increment();
        let instr = decode(op as usize);
        let instr2 = decode(op as usize);
        println!(
            "{:04X} {:04X} {:?} {:?}",
            self.registers.pc(),
            op,
            self.registers,
            instr2
        );
        self.execute_instruction(bus, instr);
    }

    fn read_immediate(&mut self, bus: &mut impl MappedHardware, size: &DataSize) -> Value {
        match self.immediate {
            Some(imm) => return imm,
            _ => (),
        }

        let immediate = match size {
            DataSize::Byte => Value::Byte(bus.read_byte(self.registers.pc()).unwrap()),
            DataSize::Word => Value::Word(bus.read_word(self.registers.pc()).unwrap()),
            DataSize::LongWord => Value::LongWord(bus.read_long(self.registers.pc()).unwrap()),
        };
        match size {
            DataSize::Byte => self.registers.displace_pc(Value::Byte(2)),
            DataSize::Word => self.registers.displace_pc(Value::Byte(2)),
            DataSize::LongWord => self.registers.displace_pc(Value::Byte(4)),
        };

        self.immediate = Some(immediate);
        immediate
    }

    fn read_addressing_mode(
        &mut self,
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
            AddressingMode::DataDirect(ref reg) => map_value(size, self.registers.data(*reg)),
            AddressingMode::Immediate => self.read_immediate(bus, size).into(),
            AddressingMode::AbsoluteAddress(DataSize::Word) => {
                let addr = self.read_immediate(bus, &DataSize::Word).into();
                bus.read_word(addr).into()
            }
            AddressingMode::AbsoluteAddress(DataSize::LongWord) => {
                let addr = self.read_immediate(bus, &DataSize::LongWord).into();
                let v = match size {
                    DataSize::Byte => bus.read_byte(addr).into(),
                    DataSize::Word => bus.read_word(addr).into(),
                    DataSize::LongWord => bus.read_long(addr).into(),
                };
                v
            }
            AddressingMode::AddressIndirectPostIncrement(ref reg) => {
                let addr = self.registers.address(*reg);
                let (incr, v) = match size {
                    DataSize::Byte => (1, bus.read_byte(addr).into()),
                    DataSize::Word => (2, bus.read_word(addr).into()),
                    DataSize::LongWord => (4, bus.read_long(addr).into()),
                };
                self.registers.set_address(*reg, addr + incr);
                v
            }
            AddressingMode::AddressIndirectDisplacement(ref reg) => {
                let addr = self.registers.address(*reg);
                let displacement: u16 = self
                    .read_addressing_mode(bus, &DataSize::Word, &AddressingMode::Immediate)
                    .into();
                let addr = (addr as i64 - displacement as u16 as i64) as u32;

                match size {
                    DataSize::Byte => bus.read_byte(addr).into(),
                    DataSize::Word => bus.read_word(addr).into(),
                    DataSize::LongWord => bus.read_long(addr).into(),
                }
            }
            AddressingMode::PCIndirectDisplacementMode => {
                let pc = self.registers.pc();
                let ext: u32 = self.read_immediate(bus, &DataSize::Word).into();

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
                    (true, DataSize::Word) => self.registers.address(idxreg) as u16 as u32,
                    (true, DataSize::LongWord) => self.registers.address(idxreg),
                    (false, DataSize::Word) => self.registers.data(idxreg) as u16 as u32,
                    (false, DataSize::LongWord) => self.registers.data(idxreg),
                    _ => unreachable!(),
                };
                Value::LongWord(pc + displacement + idx_val)
            }
            _ => unreachable!("{:?}", addressing_mode),
        }
    }

    fn write_addressing_mode(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        addressing_mode: AddressingMode,
        value: Value,
    ) {
        match addressing_mode {
            AddressingMode::DataDirect(reg) => {
                let current: u32 = self.registers.data(reg);
                let current = match size {
                    DataSize::Byte => current & 0xffff_ff00,
                    DataSize::Word => current & 0xffff_0000,
                    DataSize::LongWord => 0,
                };
                let value: u32 = value.into();
                self.registers.set_data(reg, current | value)
            }
            AddressingMode::SR => self.registers.set_ccr(value.into()),
            AddressingMode::AbsoluteAddress(DataSize::LongWord) => {
                let address = self.read_immediate(bus, &DataSize::LongWord).into();
                match size {
                    DataSize::Byte => unimplemented!("write byte not implemented"),
                    DataSize::Word => bus.write_word(address, value.into()),
                    DataSize::LongWord => bus.write_word(address, value.into()),
                };
            }
            _ => unimplemented!("{:?}", addressing_mode),
        };
    }

    fn execute_instruction(&mut self, bus: &mut impl MappedHardware, instruction: Instruction) {
        match instruction {
            Instruction::ADDQ(size, value, dest) => self.add(bus, size, value, dest),
            Instruction::ADD(size, value, dest) => self.add(bus, size, value, dest),
            Instruction::ANDI(size, source, dest) => self.andi(bus, size, source, dest),
            Instruction::ORI(size, source, dest) => self.or(bus, size, source, dest),
            Instruction::NOP => self.nop(),
            Instruction::BCC(condition, ea) => self.bcc(bus, condition, ea),
            Instruction::BRA(label) => self.bra(bus, label),
            Instruction::LEA(ea, reg) => self.lea(bus, ea, reg),
            Instruction::TST(size, ea) => self.tst(bus, size, ea),
            Instruction::MOVE(size, source, dest) => self.move_(bus, size, source, dest),
            Instruction::MOVEM(size, source, direction) => self.movem(bus, size, source, direction),
            _ => unimplemented!("{:?}", instruction),
        }
    }

    fn read_condition_code(&mut self, condition_code: Condition) -> bool {
        match condition_code {
            Condition::CC => !self.registers.ccr.contains(ConditionCode::C), // Carry Clear
            // LS => , // Lower or Same
            // CS, // Carry Set
            // LT, // Less Than
            // EQ, // EQual
            // MI, // MInus
            Condition::F => false, // False (Never true)
            Condition::NE => !self.registers.ccr.contains(ConditionCode::Z), // Not Equal
            // GE, // Greater than or Equal
            // PL, // Plus
            // GT, // Greater Than
            Condition::T => true, // True (always true)
            // HI, // HIgher
            // VC, // oVerflow Clear
            // LE, // Less than or Equal
            // VS, // oVerflow Set
            _ => unimplemented!(),
        }
    }

    //// Instruction impls

    fn nop(&self) {}

    fn bra(&mut self, bus: &mut impl MappedHardware, addressing_mode: AddressingMode) {
        let label = self.read_addressing_mode(bus, &DataSize::Byte, &addressing_mode);
        let displacement = match label {
            Value::Byte(0x00) => {
                self.read_addressing_mode(bus, &DataSize::Word, &AddressingMode::Immediate)
            }
            // Value::Byte(0xff) => self.read_addressing_mode(bus, DataSize::LongWord, AddressingMode::Immediate), // (MC68020, MC68030, MC68040 only)
            _ => label,
        };
        self.registers.displace_pc(displacement);
    }

    fn bcc(
        &mut self,
        bus: &mut impl MappedHardware,
        condition: Condition,
        addressing_mode: AddressingMode,
    ) {
        // println!("{:?}", addressing_mode);
        let displacement = self.read_addressing_mode(bus, &DataSize::Byte, &addressing_mode);
        let cond = self.read_condition_code(condition);
        if cond {
            self.registers.displace_pc(displacement)
        }
        // panic!("{:?}", label);
        // let displacement = match label {
        //     Value::Byte(0x00) => {
        //         self.read_addressing_mode(bus, &DataSize::Word, &AddressingMode::Immediate)
        //     }
        //     // Value::Byte(0xff) => self.read_addressing_mode(bus, DataSize::LongWord, AddressingMode::Immediate), // (MC68020, MC68030, MC68040 only)
        //     _ => label,
        // };
        // panic!("{:?}", displacement);
        // if self.read_condition_code(condition_code) {
        // self.registers.displace_pc(displacement);
        // }
    }

    fn add(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        value: AddressingMode,
        destination: AddressingMode,
    ) {
        let value = self.read_addressing_mode(bus, &size, &value);
        let destination_value = self.read_addressing_mode(bus, &size, &destination);

        let (result, flags) = destination_value.add_cc(size, value);
        // println!("{:?} {:?}= {:?}", destination_value, value, result);

        self.write_addressing_mode(bus, size, destination, result);
        self.registers.ccr = flags;
    }

    fn or(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        value: AddressingMode,
        destination: AddressingMode,
    ) {
        let value = self.read_addressing_mode(bus, &size, &value);
        let destination_value = self.read_addressing_mode(bus, &size, &destination);

        let (result, mut flags) = destination_value.or_cc(size, value);

        self.write_addressing_mode(bus, size, destination, result);
        flags.set(
            ConditionCode::X,
            self.registers.ccr.contains(ConditionCode::X),
        );
        self.registers.ccr = flags;
    }

    fn lea(
        &mut self,
        bus: &mut impl MappedHardware,
        addressing_mode: AddressingMode,
        register: AddressingMode,
    ) {
        let address = self.read_addressing_mode(bus, &DataSize::LongWord, &addressing_mode);
        if let AddressingMode::AddressDirect(direct) = register {
            self.registers.set_address(direct, address.into());
        };
    }

    fn move_(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        source: AddressingMode,
        dest: AddressingMode,
    ) {
        let val = self.read_addressing_mode(bus, &size, &source);
        self.write_addressing_mode(bus, size, dest, val);
    }

    fn movem(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        source: AddressingMode,
        direction: u8,
    ) {
        let mask: u16 = self
            .read_addressing_mode(bus, &DataSize::Word, &AddressingMode::Immediate)
            .into();

        let (address_mask, data_mask) = match source {
            AddressingMode::AddressIndirectPreDecrement(_) => (mask & 0xff, mask >> 8),
            _ => (mask >> 8, mask & 0xff),
        };

        for i in 0..8 {
            if data_mask & (1 << i) != 0 {
                let val: u32 = self.read_addressing_mode(bus, &size, &source).into();
                self.registers.set_data(i, val);
            }
        }
        for i in 0..8 {
            if address_mask & (1 << i) != 0 {
                let val: u32 = self.read_addressing_mode(bus, &size, &source).into();
                self.registers.set_address(i, val);
            }
        }
    }

    fn andi(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        source: AddressingMode,
        dest: AddressingMode,
    ) {
        let val: u32 = self.read_addressing_mode(bus, &size, &source).into();
        let dest_val: u32 = self.read_addressing_mode(bus, &size, &dest).into();
        let result: u32 = dest_val & val;
        let result_value = Value::from_raw(size, result);
        self.registers
            .ccr
            .set(ConditionCode::N, is_negative(&size, result_value));
        self.registers.ccr.remove(ConditionCode::V);
        self.registers.ccr.remove(ConditionCode::C);
        self.write_addressing_mode(bus, size, dest, result_value.into());
    }

    fn tst(&mut self, bus: &mut impl MappedHardware, size: DataSize, ea: AddressingMode) {
        let address = self.read_addressing_mode(bus, &size, &ea);
        let value = read_memory(bus, size, address);

        self.registers.ccr.remove(ConditionCode::V);
        self.registers.ccr.remove(ConditionCode::C);

        self.registers
            .ccr
            .set(ConditionCode::Z, 0u32 == value.into());
        self.registers
            .ccr
            .set(ConditionCode::N, 0i32 > value.into());
    }
}

fn read_memory(bus: &mut impl MappedHardware, size: DataSize, address: Value) -> Value {
    let size2 = size.clone();
    match size {
        DataSize::Byte => Value::Byte(bus.read_byte(address.into()).unwrap()),
        DataSize::Word => Value::Word(bus.read_word(address.into()).unwrap()),
        DataSize::LongWord => Value::LongWord(bus.read_long(address.into()).unwrap()),
        _ => unreachable!("{:?} {:?}", size2, address),
    }
}

fn is_negative(size: &DataSize, value: Value) -> bool {
    match (size, value) {
        (DataSize::Byte, Value::Byte(val)) => (val as i8) < 0,
        (DataSize::Word, Value::Word(val)) => (val as i16) < 0,
        (DataSize::LongWord, Value::LongWord(val)) => (val as i32) < 0,
        _ => unreachable!(),
    }
}
