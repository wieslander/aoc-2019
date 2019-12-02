pub struct Program {
    ip: usize,
    memory: Vec<usize>,
    halted: bool,
}

impl Program {
    pub fn new(initial_memory: &Vec<usize>) -> Program {
        Program {
            ip: 0,
            memory: initial_memory.clone(),
            halted: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn output(&self) -> usize {
        self.value(0)
    }

    fn value(&self, addr: usize) -> usize {
        self.memory[addr]
    }

    fn step(&mut self) {
        let opcode = self.stepval();
        match opcode {
            1 => self.add(),
            2 => self.mult(),
            99 => self.halt(),
            _ => panic!("Unknown opcode: {}", opcode),
        };
    }

    fn stepval(&mut self) -> usize {
        let value = self.memory[self.ip];
        self.ip += 1;
        value
    }

    fn add(&mut self) {
        let addr0 = self.stepval();
        let addr1 = self.stepval();
        let dst = self.stepval();
        self.memory[dst] = self.value(addr0) + self.value(addr1);
    }

    fn mult(&mut self) {
        let addr0 = self.stepval();
        let addr1 = self.stepval();
        let dst = self.stepval();
        self.memory[dst] = self.value(addr0) * self.value(addr1);
    }

    fn halt(&mut self) {
        self.halted = true;
        self.ip += 1;
    }
}
