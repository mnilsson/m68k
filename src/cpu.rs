use addressing_mode::AddressingMode;
use decoder::decode;
use instruction_set::Instruction;
use mapped_hardware::MappedHardware;
use registers::Registers;

#[derive(Default, Debug)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn tick<M: MappedHardware>(&mut self, bus: &M) {}

    pub fn execute_next_instruction(&mut self, bus: &impl MappedHardware) {
        let op = bus.read_word(self.registers.pc()).unwrap();
        self.registers.pc_increment();
        self.registers.pc_increment();
        let instr = decode(op as usize);
        self.execute_instruction(bus, instr);
    }

    fn execute_instruction(&mut self, bus: &impl MappedHardware, instruction: Instruction) {
        println!("{:?}", instruction);

        match instruction {
            Instruction::NOP => self.nop(),
            Instruction::BRA(ea) => self.bra(ea),
            _ => {}
        }
    }

    fn nop(&self) {}

    fn bra(&self, addressing_mode: AddressingMode) {}
}
