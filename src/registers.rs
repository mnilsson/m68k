use cpu::Value;

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
    struct SupervisorStatusRegister: u8 {
        const T = 0b1000_0000;
        const S = 0b0010_0000;

        const IM2 = 0b0000_0100;
        const IM1 = 0b0000_0010;
        const IM0 = 0b0000_0001;
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    d: [u32; 8],
    a: [u32; 8],
    pc: u32,
    sp: u32,
    pub ccr: ConditionCode,

    ssp: u32, // a7' ssp supervisor stackpointer
    system_status_register: SupervisorStatusRegister,
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

    pub fn set_sp(&mut self, new_sp: u32) {
        self.sp = new_sp;
    }

    pub fn displace_pc(&mut self, value: Value) {
        let displacement: i32 = value.into();
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
    }

    pub fn set_ccr(&mut self, value: u8) {
        self.ccr = ConditionCode::from_bits(value.into()).unwrap();
    }
}
