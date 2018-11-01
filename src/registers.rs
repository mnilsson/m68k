bitflags! {
    #[derive(Default)]
    struct ConditionCodeRegister: u8 {
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
    ccr: ConditionCodeRegister,

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
}
