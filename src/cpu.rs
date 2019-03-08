use addressing_mode::{
    read_addressing_mode, read_addressing_mode_address, write_addressing_mode, AddressingMode,
    Condition, DataSize,
};
use decoder::decode;
use instruction_set::Instruction;
use mapped_hardware::MappedHardware;
use registers::{ConditionCode, Registers, SupervisorStatusRegister};

use std::ops::Add;
use value::Value;

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

const ADDRESS_BUS_MASK: u32 = 0x7f_ffff;

// directions
// in = into cpu
// out = out from cpu

#[derive(Default, Debug)]
pub struct Cpu {
    pub registers: Registers,
    instruction_step: InstructionStep,
    instruction_clock: usize,

    immediate: Option<Value>,

    interrupt_requests: Vec<(usize, Option<usize>)>, // (level, Option<address>)
    stopped: bool,
    pub debug: bool,
}

impl Cpu {
    pub fn tick<M: MappedHardware>(&mut self, bus: &M) {}

    pub fn reset(&mut self, bus: &mut impl MappedHardware) {
        self.set_pc(0);
        let new_sp = self.read_immediate(bus, &DataSize::LongWord).into();

        self.set_sp(new_sp);

        self.immediate = None;
        let new_pc = self.read_immediate(bus, &DataSize::LongWord).into();
        self.registers.set_pc(new_pc);
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.registers.set_pc(new_pc);
    }

    pub fn set_sp(&mut self, new_pc: u32) {
        self.registers.set_sp(new_pc);
    }

    pub fn request_auto_interrupt(&mut self, interrupt: usize) {
        self.interrupt_requests.push((interrupt, None))
    }

    pub fn request_interrupt(&mut self, interrupt: usize, address: usize) {
        self.interrupt_requests.push((interrupt, Some(address)))
    }

    fn run_interrupt(&mut self, bus: &mut impl MappedHardware) {
        if self.interrupt_requests.is_empty() {
            return;
        }

        let (irqlevel, address) = self.interrupt_requests[0];
        let address = match address {
            None => {
                let intermediate = (irqlevel * 4) + 60;
                bus.read_long(intermediate as u32).unwrap() as usize
            }
            Some(address) => address + 0,
        };

        let pc = self.registers.pc();
        self.push_stack(bus, DataSize::LongWord, Value::LongWord(pc));
        let ccr = self.registers.complete_ccr();
        self.push_stack(bus, DataSize::Word, Value::Word(ccr));

        if irqlevel + 0 <= 7 {
            let bits = self.registers.system_status_register.bits() & 0xf8;
            self.registers.system_status_register =
                SupervisorStatusRegister::from_bits_truncate(bits | (irqlevel + 0) as u8);
        }

        self.interrupt_requests.remove(0);
        self.set_pc(address as u32)
        self.stopped = false;
    }

    pub fn execute_next_instruction(&mut self, bus: &mut impl MappedHardware) {
        self.run_interrupt(bus);
        self.immediate = None;
        let pc = self.registers.pc();
        let op = bus.read_word(self.registers.pc()).unwrap();

        if self.stopped {
            return;
        }

        self.registers.pc_increment();
        self.registers.pc_increment();
        let instr = decode(op as usize);
        let instr2 = decode(op as usize);
        if self.debug {
            println!("{:04X} {:04X} {:?} {:?}", pc, op, self.registers, instr2);
        }
        self.execute_instruction(bus, instr);
    }

    pub fn read_immediate(&mut self, bus: &mut impl MappedHardware, size: &DataSize) -> Value {
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

    fn push_stack(&mut self, bus: &mut impl MappedHardware, size: DataSize, value: Value) {
        match size {
            DataSize::Word => {
                let sp = self.registers.sp();
                self.registers.set_sp(sp - 2);
                bus.write_word(self.registers.sp(), value.into());
            }
            DataSize::LongWord => {
                let sp = self.registers.sp();
                self.registers.set_sp(sp - 4);
                bus.write_long(self.registers.sp(), value.into());
            }
            _ => unreachable!(),
        }
    }

    fn read_addressing_mode(
        &mut self,
        bus: &mut impl MappedHardware,
        size: &DataSize,
        addressing_mode: &AddressingMode,
    ) -> Value {
        read_addressing_mode(self, bus, size, addressing_mode)
    }

    fn write_addressing_mode(
        &mut self,
        bus: &mut impl MappedHardware,
        size: &DataSize,
        addressing_mode: &AddressingMode,
        value: Value,
    ) {
        write_addressing_mode(self, bus, size, addressing_mode, value)
    }

    fn execute_instruction(&mut self, bus: &mut impl MappedHardware, instruction: Instruction) {
        match instruction {
            Instruction::ADDQ(size, value, dest) => self.add(bus, size, value, dest),
            Instruction::ADD(size, value, dest) => self.add(bus, size, value, dest),
            Instruction::SUBI(size, value, dest) => self.sub(bus, size, value, dest),
            Instruction::SUBA(size, value, dest) => self.sub(bus, size, value, dest),
            Instruction::SUBQ(size, value, dest) => self.sub(bus, size, value, dest),
            Instruction::SUB(size, value, dest) => self.sub(bus, size, value, dest),
            Instruction::ADDI(size, value, dest) => self.add(bus, size, value, dest),
            Instruction::AND(size, source, dest) => self.and(bus, size, source, dest),
            Instruction::ANDI(size, source, dest) => self.and(bus, size, source, dest),
            Instruction::CMP(size, source, dest) => self.cmp(bus, size, source, dest),
            Instruction::CMPI(size, ea) => self.cmp(bus, size, AddressingMode::Immediate, ea),
            Instruction::DB(condition, source, ea) => self.db(bus, condition, source, ea),
            Instruction::ORI(size, source, dest) => self.or(bus, size, source, dest),
            Instruction::OR(size, source, dest) => self.or(bus, size, source, dest),
            Instruction::EORI(size, source, dest) => self.eor(bus, size, source, dest),
            Instruction::EOR(size, source, dest) => self.eor(bus, size, source, dest),
            Instruction::NOP => self.nop(),
            Instruction::BCC(size, condition, ea) => self.bcc(bus, size, condition, ea),
            Instruction::BRA(label) => self.bra(bus, label),
            Instruction::LEA(ea, reg) => self.lea(bus, ea, reg),
            Instruction::TST(size, ea) => self.tst(bus, size, ea),
            Instruction::BTST(size, bit, ea) => self.btst(bus, size, bit, ea),
            Instruction::BSET(size, bit, ea) => self.btst(bus, size, bit, ea),
            Instruction::MOVE(size, source, dest) => self.move_(bus, size, source, dest),
            Instruction::MOVEQ(size, source, dest) => self.move_(bus, size, source, dest),
            Instruction::MOVEM(size, source, direction) => self.movem(bus, size, source, direction),
            Instruction::JMP(label) => self.jmp(bus, label),
            Instruction::CLR(size, destination) => self.clr(bus, size, destination),
            Instruction::JSR(ea) => self.jsr(bus, ea),
            Instruction::RTS => self.rts(bus),
            Instruction::RTE => self.rte(bus),
            Instruction::BSR(label) => self.bsr(bus, label),
            Instruction::LSLD(size, count, dest) => self.lsl(bus, size, count, dest),
            Instruction::LSRD(size, count, dest) => self.lsr(bus, size, count, dest),
            Instruction::LINK(reg, displacement) => {
                self.link(bus, DataSize::Word, reg, displacement)
            }
            Instruction::PEA(ea) => self.pea(bus, ea),
            Instruction::STOP(ccr) => self.stop(bus, ccr),
            _ => unimplemented!("{:?}", instruction),
        }
    }

    fn read_condition_code(&mut self, condition_code: Condition) -> bool {
        match condition_code {
            Condition::CC => !self.registers.ccr.contains(ConditionCode::C), // Carry Clear
            LS => {
                self.registers.ccr.contains(ConditionCode::N)
                    || self.registers.ccr.contains(ConditionCode::Z)
            } // Lower or Same
            // CS, // Carry Set
            // LT, // Less Than
            Condition::EQ => self.registers.ccr.contains(ConditionCode::Z), // EQual
            // MI, // MInus
            Condition::F => false, // False (Never true)
            Condition::NE => !self.registers.ccr.contains(ConditionCode::Z), // Not Equal
            // Condition::GE, // Greater than or Equal
            // PL, // Plus
            // GT, // Greater Than
            Condition::T => true, // True (always true)
            // HI, // HIgher
            // VC, // oVerflow Clear
            // LE, // Less than or Equal
            // VS, // oVerflow Set
            _ => unimplemented!("{:?}", condition_code),
        }
    }

    //// Instruction impls

    fn nop(&self) {}

    fn stop(&mut self, bus: &mut impl MappedHardware, ccr: AddressingMode) {
        let ccr: u16 = self.read_addressing_mode(bus, &DataSize::Word, &ccr).into();
        if self
            .registers
            .system_status_register
            .contains(SupervisorStatusRegister::S)
        {
            self.registers.set_complete_ccr(ccr);
        }
        self.stopped = true;
    }

    fn bra(&mut self, bus: &mut impl MappedHardware, addressing_mode: AddressingMode) {
        let before_pc = self.registers.pc();
        let label = self.read_addressing_mode(bus, &DataSize::Word, &addressing_mode);
        // let displacement = match label {
        //     Value::Byte(0x00) => {
        //         self.read_addressing_mode(bus, &DataSize::Word, &AddressingMode::Immediate)
        //     }
        //     // Value::Byte(0xff) => self.read_addressing_mode(bus, DataSize::LongWord, AddressingMode::Immediate), // (MC68020, MC68030, MC68040 only)
        //     _ => label,
        // };

        let label = self.read_addressing_mode(bus, &size, &addressing_mode);
        self.registers.set_pc(before_pc);
        self.registers.displace_pc(label);
    }

    fn bcc(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        condition: Condition,
        addressing_mode: AddressingMode,
    ) {
        let displacement = self.read_addressing_mode(bus, &size, &addressing_mode);
        let cond = self.read_condition_code(condition);
        if cond {
            match size {
                DataSize::Byte => (),
                DataSize::Word => self.registers.displace_pc(Value::Byte((0 - 2) as u8)),
                DataSize::LongWord => self.registers.displace_pc(Value::Byte((0 - 4) as u8)),
            }
            self.registers.displace_pc(displacement)
        }
    }

    fn db(
        &mut self,
        bus: &mut impl MappedHardware,
        condition: Condition,
        data: AddressingMode,
        displacement: AddressingMode,
    ) {
        let before_pc = self.registers.pc();

        let cond = self.read_condition_code(condition);
        let data_val: u16 = self
            .read_addressing_mode(bus, &DataSize::Word, &data)
            .into();
        self.immediate = None;

        let label = self.read_addressing_mode(bus, &DataSize::Word, &displacement);
        self.immediate = None;
        if !cond {
            let mut data_val = data_val as i16;
            data_val -= 1;
            self.write_addressing_mode(bus, &DataSize::Word, &data, Value::Word(data_val as u16));
            if data_val != -1 {
                self.registers.set_pc(before_pc);
                self.registers.displace_pc(label);
            }
        }
    }

    fn jmp(&mut self, bus: &mut impl MappedHardware, label: AddressingMode) {
        let label_data: u32 = read_addressing_mode_address(self, bus, &DataSize::LongWord, &label);

        self.set_pc(label_data);
    }

    fn jsr(&mut self, bus: &mut impl MappedHardware, label: AddressingMode) {
        let dest_pc: u32 =
            read_addressing_mode_address(self, bus, &DataSize::LongWord, &label).into();
        let next_pc = self.registers.pc();
        self.push_stack(bus, DataSize::LongWord, Value::LongWord(next_pc));
        self.set_pc(dest_pc);
    }

    fn bsr(&mut self, bus: &mut impl MappedHardware, addressing_mode: AddressingMode) {
        let before_pc = self.registers.pc();

        let label = self.read_addressing_mode(bus, &DataSize::Word, &addressing_mode);

        self.push_stack(bus, DataSize::LongWord, Value::LongWord(before_pc - 2));
        self.registers.set_pc(before_pc);
        self.registers.displace_pc(label);
    }

    fn rts(&mut self, bus: &mut impl MappedHardware) {
        let sp = self.registers.sp();
        let new_addr = bus.read_long(sp);
        self.registers.set_pc(new_addr.unwrap());
        self.registers.set_sp(sp + 4);
    }

    fn rte(&mut self, bus: &mut impl MappedHardware) {
        let sp = self.registers.sp();
        let new_addr = bus.read_word(sp);
        self.registers.set_complete_ccr(new_addr.unwrap());
        self.registers.set_sp(sp + 2);

        let sp = self.registers.sp();
        let new_addr = bus.read_long(sp);
        self.registers.set_pc(new_addr.unwrap());
        self.registers.set_sp(sp + 4);
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

        self.write_addressing_mode(bus, &size, &destination, result);
        self.registers.ccr = flags;
    }

    fn sub(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        value: AddressingMode,
        destination: AddressingMode,
    ) {
        let value = self.read_addressing_mode(bus, &size, &value);
        let destination_value = self.read_addressing_mode(bus, &size, &destination);

        let (result, flags) = destination_value.sub_cc(size, value);

        self.write_addressing_mode(bus, &size, &destination, result);
        self.registers.ccr = flags;
    }

    fn or(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        value: AddressingMode,
        destination: AddressingMode,
    ) {
        let destination = match destination {
            AddressingMode::Immediate => AddressingMode::CCR,
            _ => destination,
        };
        let value = self.read_addressing_mode(bus, &size, &value);
        let destination_value = self.read_addressing_mode(bus, &size, &destination);

        let (result, mut flags) = destination_value.or_cc(size, value);

        self.write_addressing_mode(bus, &size, &destination, result);
        flags.set(
            ConditionCode::X,
            self.registers.ccr.contains(ConditionCode::X),
        );
        self.registers.ccr = flags;
    }

    fn eor(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        value: AddressingMode,
        destination: AddressingMode,
    ) {
        let destination = match destination {
            AddressingMode::Immediate => AddressingMode::CCR,
            _ => destination,
        };
        let value = self.read_addressing_mode(bus, &size, &value);
        let destination_value = self.read_addressing_mode(bus, &size, &destination);

        let (result, mut flags) = destination_value.eor_cc(size, value);

        self.write_addressing_mode(bus, &size, &destination, result);
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
        let address =
            read_addressing_mode_address(self, bus, &DataSize::LongWord, &addressing_mode);
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
        self.immediate = None;
        self.write_addressing_mode(bus, &size, &dest, val);
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
        self.immediate = None;

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

    fn and(
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
        self.write_addressing_mode(bus, &size, &dest, result_value.into());
    }

    fn tst(&mut self, bus: &mut impl MappedHardware, size: DataSize, ea: AddressingMode) {
        let value = self.read_addressing_mode(bus, &size, &ea);
        // let value = read_memory(bus, size, address);

        self.registers.ccr.remove(ConditionCode::V);
        self.registers.ccr.remove(ConditionCode::C);

        self.registers
            .ccr
            .set(ConditionCode::Z, 0u32 == value.into());
        self.registers
            .ccr
            .set(ConditionCode::N, 0i32 > value.into());
    }

    fn btst(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        bit: AddressingMode,
        ea: AddressingMode,
    ) {
        let bit: u32 = self.read_addressing_mode(bus, &size, &bit).into();
        self.immediate = None;
        let ea: u32 = self.read_addressing_mode(bus, &size, &ea).into();

        self.registers.ccr.set(ConditionCode::Z, ea & bit == 0);
    }

    fn bset(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        bit: AddressingMode,
        ea: AddressingMode,
    ) {
        let bit: u32 = self.read_addressing_mode(bus, &size, &bit).into();
        self.immediate = None;
        let ea: u32 = self.read_addressing_mode(bus, &size, &ea).into();

        self.registers.ccr.set(ConditionCode::Z, ea & bit == 0);
    }

    fn clr(&mut self, bus: &mut impl MappedHardware, size: DataSize, destination: AddressingMode) {
        self.write_addressing_mode(bus, &size, &destination, Value::from_raw(size, 0));

        self.registers.ccr.set(ConditionCode::N, false);
        self.registers.ccr.set(ConditionCode::Z, true);
        self.registers.ccr.set(ConditionCode::V, false);
        self.registers.ccr.set(ConditionCode::C, false);
    }

    fn cmp(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        source: AddressingMode,
        dest: AddressingMode,
    ) {
        let val: u32 = self.read_addressing_mode(bus, &size, &source).into();
        let dest_val: u32 = self.read_addressing_mode(bus, &size, &dest).into();
        let (result, v) = dest_val.overflowing_sub(val);
        let result_value = Value::from_raw(size, result);
        self.registers
            .ccr
            .set(ConditionCode::N, is_negative(&size, result_value));
        self.registers.ccr.set(ConditionCode::Z, result == 0);
        self.registers.ccr.set(ConditionCode::V, v);
        self.registers.ccr.set(ConditionCode::C, v);
    }

    fn link(
        &mut self,
        bus: &mut impl MappedHardware,
        displacement_size: DataSize,
        reg: AddressingMode,
        displacement: AddressingMode,
    ) {
        let size = DataSize::LongWord;
        let val = read_addressing_mode_address(self, bus, &size, &reg);
        self.push_stack(bus, size, Value::LongWord(val));
        let sp = self.registers.sp();
        self.write_addressing_mode(bus, &size, &reg, Value::LongWord(sp));
        let displacement_val = self.read_addressing_mode(bus, &displacement_size, &displacement);
        self.registers.displace_sp(displacement_val);
    }

    fn pea(&mut self, bus: &mut impl MappedHardware, ea: AddressingMode) {
        let val = read_addressing_mode_address(self, bus, &DataSize::LongWord, &ea);
        self.push_stack(bus, DataSize::LongWord, Value::LongWord(val));
    }

    fn lsl(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        count: AddressingMode,
        destination: AddressingMode,
    ) {
        let count: u32 = match count {
            AddressingMode::DataDirect(_) => {
                let c: u32 = self
                    .read_addressing_mode(bus, &DataSize::LongWord, &count)
                    .into();
                c % 64
            }
            _ => {
                let mut c: u32 = self.read_addressing_mode(bus, &size, &count).into();
                if c == 0 {
                    c = 8
                }
                c
            }
        };
        let dest_val = self.read_addressing_mode(bus, &size, &destination);

        let (result, mut ccr) = shift_left(size, count, dest_val);

        if count == 0 {
            ccr.set(
                ConditionCode::X,
                self.registers.ccr.contains(ConditionCode::X),
            );
        }

        self.registers.set_ccr(ccr.bits());
        self.write_addressing_mode(bus, &size, &destination, result);
    }

    fn lsr(
        &mut self,
        bus: &mut impl MappedHardware,
        size: DataSize,
        count: AddressingMode,
        destination: AddressingMode,
    ) {
        let count: u32 = match count {
            AddressingMode::DataDirect(_) => {
                let c: u32 = self
                    .read_addressing_mode(bus, &DataSize::LongWord, &count)
                    .into();
                c % 64
            }
            _ => {
                let mut c: u32 = self.read_addressing_mode(bus, &size, &count).into();
                if c == 0 {
                    c = 8
                }
                c
            }
        };
        let dest_val = self.read_addressing_mode(bus, &size, &destination);

        let (result, mut ccr) = shift_right(size, count, dest_val);

        if count == 0 {
            ccr.set(
                ConditionCode::X,
                self.registers.ccr.contains(ConditionCode::X),
            );
        }

        self.registers.set_ccr(ccr.bits());
        self.write_addressing_mode(bus, &size, &destination, result);
    }
}

fn shift_right(size: DataSize, count: u32, value: Value) -> (Value, ConditionCode) {
    let (result, bit) = match size {
        DataSize::Byte => {
            let value: u8 = value.into();
            let b = (value >> (count - 1)) & 0x1 != 0;
            (Value::Byte(value >> count), b)
        }
        DataSize::Word => {
            let value: u16 = value.into();
            let b = (value >> (count - 1)) & 0x1 != 0;
            (Value::Word(value >> count), b)
        }
        DataSize::LongWord => {
            let value: u32 = value.into();
            let b = (value >> (count - 1)) & 0x1 != 0;
            (Value::LongWord(value >> count), b)
        }
    };

    let mut ccr = ConditionCode::empty();

    ccr.set(ConditionCode::N, is_negative(&size, result));
    ccr.set(ConditionCode::Z, is_zero(&size, result));
    if bit {
        ccr |= ConditionCode::X;
        if count > 0 {
            ccr |= ConditionCode::C;
        }
        // ccr |=
    }

    (result, ccr)
}

fn shift_left(size: DataSize, count: u32, value: Value) -> (Value, ConditionCode) {
    let (result, bit) = match size {
        DataSize::Byte => {
            let value: u8 = value.into();
            let b = (value << (count - 1)) & 0x80 != 0;
            (Value::Byte(value << count), b)
        }
        DataSize::Word => {
            let value: u16 = value.into();
            let b = (value << (count - 1)) & 0x8000 != 0;
            (Value::Word(value << count), b)
        }
        DataSize::LongWord => {
            let value: u32 = value.into();
            let b = (value << (count - 1)) & 0x8000_0000 != 0;
            (Value::LongWord(value << count), b)
        }
    };

    let mut ccr = ConditionCode::empty();

    ccr.set(ConditionCode::N, is_negative(&size, result));
    ccr.set(ConditionCode::Z, is_zero(&size, result));
    if bit {
        ccr |= ConditionCode::X;
        if count > 0 {
            ccr |= ConditionCode::C;
        }
        // ccr |=
    }

    (result, ccr)
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

fn is_zero(size: &DataSize, value: Value) -> bool {
    match (size, value) {
        (DataSize::Byte, Value::Byte(val)) => (val as i8) == 0,
        (DataSize::Word, Value::Word(val)) => (val as i16) == 0,
        (DataSize::LongWord, Value::LongWord(val)) => (val as i32) == 0,
        _ => unreachable!(),
    }
}

fn test_bit(val: u32, bit: u32) -> bool {
    let mask = 1 << bit;
    val & mask == mask
}

fn set_bit(val: u32, bit: u32) -> u32 {
    let mask = 1 << bit;

    if val & mask == mask {
        val | mask
    } else {
        val ^ mask
    }
}

#[test]
fn test_test_bit() {
    assert_eq!(true, test_bit(0b1111_1111, 7));
    assert_eq!(false, test_bit(0b0111_1111, 7));
}

fn test_set_bit() {
    assert_eq!(0b1111_1111, set_bit(0b1111_1111, 7));
    assert_eq!(0b1111_1111, set_bit(0b0111_1111, 7));
    assert_eq!(0b0100_0000, set_bit(0b0000_0000, 6));
}
