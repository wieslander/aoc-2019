use std::collections::HashMap;

#[derive(Clone)]
pub struct Program {
    ip: usize,
    memory: HashMap<usize, i64>,
    halted: bool,
    jumped: bool,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    relative_base: i64,
}

enum Param {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl Param {
    fn new(value: i64, mode_code: i64) -> Param {
        match mode_code {
            0 => Param::Position(value as usize),
            1 => Param::Immediate(value),
            2 => Param::Relative(value),
            _ => panic!("Unknown parameter mode {}", mode_code),
        }
    }
}

struct Instruction {
    opcode: i64,
    params: Vec<Param>,
}

impl Instruction {
    pub fn new(opcode: i64) -> Instruction {
        Instruction {
            opcode,
            params: vec![],
        }
    }

    pub fn len(&self) -> usize {
        match self.opcode {
            1 => 4,
            2 => 4,
            3 => 2,
            4 => 2,
            5 => 3,
            6 => 3,
            7 => 4,
            8 => 4,
            9 => 2,
            99 => 1,
            _ => panic!("Unknown opcode: {}", self.opcode),
        }
    }

    pub fn add_param(&mut self, param: Param) {
        self.params.push(param);
    }
}

impl Program {
    pub fn new(initial_memory: &Vec<i64>) -> Program {
        let mut memory = HashMap::new();
        for (addr, &value) in initial_memory.iter().enumerate() {
            memory.insert(addr, value);
        }

        Program {
            ip: 0,
            memory,
            halted: false,
            jumped: false,
            inputs: vec![],
            outputs: vec![],
            relative_base: 0,
        }
    }

    pub fn reset(&mut self, initial_memory: &Vec<i64>) {
        let mut memory = HashMap::new();
        for (addr, &value) in initial_memory.iter().enumerate() {
            memory.insert(addr, value);
        }

        self.ip = 0;
        self.memory = memory;
        self.halted = false;
        self.jumped = false;
        self.inputs.clear();
        self.outputs.clear();
        self.relative_base = 0;
    }

    pub fn is_running(&self) -> bool {
        !self.halted
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn pause_on_output(&mut self) -> Option<i64> {
        while self.outputs.len() == 0 && !self.halted {
            self.step();
        }

        self.pop_output()
    }

    pub fn needs_input(&self) -> bool {
        let instruction = self.get_instruction();
        instruction.opcode == 3 && self.inputs.is_empty()
    }

    pub fn has_output(&self) -> bool {
        self.outputs.len() > 0
    }

    pub fn read(&self, addr: usize) -> i64 {
        match self.memory.get(&addr) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn set_input(&mut self, input: i64) {
        self.inputs.insert(0, input);
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        self.outputs.pop()
    }

    fn raw_param(&self, offset: usize) -> i64 {
        self.read(self.ip + offset)
    }

    fn write(&mut self, addr_param: &Param, value: i64) {
        let addr = self.get_write_address(&addr_param);
        self.memory.insert(addr, value);
    }

    fn get_write_address(&self, param: &Param) -> usize {
        match param {
            Param::Position(addr) => *addr,
            Param::Relative(offset) => (self.relative_base + offset) as usize,
            _ => panic!("Unsupported parameter mode for write"),
        }
    }

    fn read_param(&self, param: &Param) -> i64 {
        match param {
            Param::Position(addr) => self.read(*addr),
            Param::Immediate(value) => *value,
            Param::Relative(offset) => {
                let addr = (self.relative_base + offset) as usize;
                self.read(addr)
            },
        }
    }

    fn get_instruction(&self) -> Instruction {
        let raw_opcode = self.read(self.ip);
        let opcode = raw_opcode % 100;
        let mut instruction = Instruction::new(opcode);
        let param_modifiers = raw_opcode - opcode;

        for i in 1..instruction.len() {
            let value = self.raw_param(i);
            let mode_code = (param_modifiers / 10i64.pow(i as u32 + 1)) % 10;
            let param = Param::new(value, mode_code);
            instruction.add_param(param);
        }

        instruction
    }

    pub fn step(&mut self) {
        let instruction = self.get_instruction();
        let opcode = instruction.opcode;
        match opcode {
            1 => self.add(&instruction.params),
            2 => self.mult(&instruction.params),
            3 => self.input(&instruction.params),
            4 => self.output(&instruction.params),
            5 => self.jump_if_true(&instruction.params),
            6 => self.jump_if_false(&instruction.params),
            7 => self.lt(&instruction.params),
            8 => self.eq(&instruction.params),
            9 => self.add_relbase(&instruction.params),
            99 => self.halt(),
            _ => panic!("Unknown opcode: {}", opcode),
        }
        if !self.jumped {
            self.ip += instruction.len();
        }
        self.jumped = false;
    }

    fn add(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);
        self.write(&params[2], val0 + val1);
    }

    fn mult(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);
        self.write(&params[2], val0 * val1);
    }

    fn input(&mut self, params: &Vec<Param>) {
        match self.inputs.pop() {
            Some(input) => self.write(&params[0], input),
            None => panic!("No input available"),
        }
    }

    fn output(&mut self, params: &Vec<Param>) {
        let val = self.read_param(&params[0]);
        self.outputs.push(val);
    }

    fn jump_if_true(&mut self, params: &Vec<Param>) {
        let val = self.read_param(&params[0]);
        if val != 0 {
            self.jump(&params[1]);
        }
    }

    fn jump_if_false(&mut self, params: &Vec<Param>) {
        let val = self.read_param(&params[0]);
        if val == 0 {
            self.jump(&params[1]);
        }
    }

    fn jump(&mut self, param: &Param) {
        self.ip = self.read_param(param) as usize;
        self.jumped = true;
    }

    fn lt(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);

        self.write(&params[2], (val0 < val1) as i64);
    }

    fn eq(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);

        self.write(&params[2], (val0 == val1) as i64);
    }

    fn add_relbase(&mut self, params: &Vec<Param>) {
        let offset = self.read_param(&params[0]);
        self.relative_base += offset;
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}
