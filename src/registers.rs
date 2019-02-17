use value::Value;

bitflags! {
    #[derive(Default)]
    pub struct ConditionCode: u8 {
        const X = 0b1_0000;
        const N = 0b0_1000;
        const Z = 0b0_0100;
        const V = 0b0_0010;
        const C = 0b0_0001;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct SupervisorStatusRegister: u8 {
        const T = 0b1000_0000;
        const S = 0b0010_0000;
        const M = 0b0001_0000;


        const IM2 = 0b0000_0100;
        const IM1 = 0b0000_0010;
        const IM0 = 0b0000_0001;
    }
}

enum StackPointer {
    USP,
    ISP,
    MSP,
}

#[derive(Default)]
pub struct Registers {
    d: [u32; 8],
    a: [u32; 8],
    pc: u32,
    sp: u32, // active stack pointer a7
    pub ccr: ConditionCode,

    usp: u32, // user stack pointer
    isp: u32, // interrupt stack pointer
    msp: u32, // master stack pointer

    system_status_register: SupervisorStatusRegister,
}
use std::fmt;

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.d.len() {
            write!(f, "d{}:{:08X} ", i + 1, self.d[i]);
        }
        for i in 0..self.a.len() {
            write!(f, "a{}:{:08X} ", i + 1, self.a[i]);
        }

        write!(f, "pc: {:08X} ccr: {:?}, sr: {:?}", self.pc, self.ccr, self.system_status_register);
        write!(f, "")
    }
}
impl Registers {
    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn pc_increment(&mut self) {
        self.pc += 1
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }

    pub fn sp(&self) -> u32 {
        self.a[7]
    }

    pub fn set_sp(&mut self, new_sp: u32) {
        self.a[7] = new_sp;
        self.save_current_stack_pointer();
    }

    fn get_current_stack_pointer(&self) -> StackPointer {
        if self
            .system_status_register
            .contains(SupervisorStatusRegister::S)
        {
            if self
                .system_status_register
                .contains(SupervisorStatusRegister::M)
            {
                StackPointer::ISP
            } else {
                StackPointer::MSP
            }
        } else {
            StackPointer::MSP
        }
    }

    pub fn set_usp(&mut self, new_value: u32) {
        self.usp = new_value;
    }

    pub fn displace_sp(&mut self, value: Value) {
        let displacement: i32 = match value {
            Value::Byte(byte) => byte as i8 as i32,
            Value::Word(word) => word as i16 as i32,
            Value::LongWord(long) => long as i32,
        };
        let sp = self.sp();
        self.set_sp(((sp as i64) + displacement as i64) as u32);
    }

    pub fn displace_pc(&mut self, value: Value) {
        let displacement: i32 = match value {
            Value::Byte(byte) => byte as i8 as i32,
            Value::Word(word) => word as i16 as i32,
            Value::LongWord(_) => unreachable!("Long displacement not valid"),
        };
        self.pc = ((self.pc as i64) + displacement as i64) as u32;
    }

    pub fn data(&mut self, reg: usize) -> u32 {
        self.d[reg]
    }

    pub fn set_data(&mut self, reg: usize, value: u32) {
        self.d[reg] = value;
    }

    pub fn address(&mut self, reg: usize) -> u32 {
        self.a[reg]
    }

    pub fn set_address(&mut self, reg: usize, value: u32) {
        self.a[reg] = value;
        if reg == 7 {
            self.save_current_stack_pointer();
        }
    }

    pub fn set_ccr(&mut self, value: u8) {
        self.save_current_stack_pointer();
        self.ccr = ConditionCode::from_bits(value.into()).unwrap();
        self.apply_stack_pointer();
    }

    pub fn sr(&self) -> u16 {
        let ccr: u16 = self.ccr.bits().into();
        let sr: u16 = self.system_status_register.bits().into();
        (sr << 8) | ccr
    }

    fn save_current_stack_pointer(&mut self) {
        match self.get_current_stack_pointer() {
            StackPointer::ISP => self.isp = self.a[7],
            StackPointer::MSP => self.msp = self.a[7],
            StackPointer::USP => self.usp = self.a[7],
        };
    }
    fn apply_stack_pointer(&mut self) {
        match self.get_current_stack_pointer() {
            StackPointer::ISP => self.a[7] = self.isp,
            StackPointer::MSP => self.a[7] = self.msp,
            StackPointer::USP => self.a[7] = self.usp,
        };
    }
}
