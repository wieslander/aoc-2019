pub struct Program {
    ip: usize,
    memory: Vec<i64>,
    halted: bool,
    jumped: bool,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

struct Param {
    value: i64,
    immediate: bool,
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
            99 => 1,
            _ => panic!("Unknown opcode: {}", self.opcode),
        }
    }

    pub fn add_param(&mut self, value: i64, immediate: bool) {
        self.params.push(Param { value, immediate });
    }
}

impl Program {
    pub fn new(initial_memory: &Vec<i64>) -> Program {
        Program {
            ip: 0,
            memory: initial_memory.clone(),
            halted: false,
            jumped: false,
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn reset(&mut self, initial_memory: &Vec<i64>) {
        self.ip = 0;
        self.memory = initial_memory.clone();
        self.halted = false;
        self.jumped = false;
        self.inputs.clear();
        self.outputs.clear();
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

    pub fn read(&self, addr: usize) -> i64 {
        self.memory[addr]
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

    fn write(&mut self, addr: usize, value: i64) {
        self.memory[addr] = value;
    }

    fn read_param(&self, param: &Param) -> i64 {
        if param.immediate {
            param.value
        } else {
            self.read(param.value as usize)
        }
    }

    fn get_instruction(&self) -> Instruction {
        let raw_opcode = self.read(self.ip);
        let opcode = raw_opcode % 100;
        let mut instruction = Instruction::new(opcode);
        let param_modifiers = raw_opcode - opcode;

        for i in 1..instruction.len() {
            let value = self.raw_param(i);
            let immediate = (param_modifiers / 10i64.pow(i as u32 + 1)) % 2 == 1;

            instruction.add_param(value, immediate);
        }

        instruction
    }

    fn step(&mut self) {
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
        let dst = params[2].value as usize;
        self.write(dst, val0 + val1);
    }

    fn mult(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);
        let dst = params[2].value as usize;
        self.write(dst, val0 * val1);
    }

    fn input(&mut self, params: &Vec<Param>) {
        let dst = params[0].value as usize;

        match self.inputs.pop() {
            Some(input) => self.write(dst, input),
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
        let dst = params[2].value as usize;

        self.write(dst, (val0 < val1) as i64);
    }

    fn eq(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);
        let dst = params[2].value as usize;

        self.write(dst, (val0 == val1) as i64);
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}
