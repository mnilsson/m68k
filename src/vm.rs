use bus::Bus;
use cpu::Cpu;
use mapped_hardware::MappedHardware;
use memory::Memory;

pub struct VirtualMachine {
    cpu: Cpu,
    bus: Bus,
}

impl VirtualMachine {
    pub fn new(prg: Vec<u8>) -> VirtualMachine {
        let mut bus = Bus::default();
        let mut cpu = Cpu::default();

        VirtualMachine {
            cpu: cpu,
            bus: Bus::default(),
        }
    }

    pub fn map_hardware(&mut self, hardware: Box<MappedHardware>) {
        self.bus.map_hardware(hardware);
    }

    pub fn init(&mut self) {
        self.cpu.reset(&mut self.bus);
    }

    pub fn tick(&mut self) {
        self.cpu.execute_next_instruction(&mut self.bus);
        // println!("Cycles: {}", self.bus.cycles);
        // let bus = &self.bus;
        // if let Some(byte) = bus.read_byte(0) {
        //     println!("{:X}", byte);
        // }
    }
    pub fn run(&mut self) {
        loop {
            let bus = &self.bus;
            // self.cpu.tick(bus);
        }
    }
}
