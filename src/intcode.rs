pub struct Program {
    ip: usize,
    memory: Vec<i32>,
    halted: bool,
    jumped: bool,
    _input: Option<i32>,
}

struct Param {
    value: i32,
    immediate: bool,
}

struct Instruction {
    opcode: i32,
    params: Vec<Param>,
}

impl Instruction {
    pub fn new(opcode: i32) -> Instruction {
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

    pub fn add_param(&mut self, value: i32, immediate: bool) {
        self.params.push(Param { value, immediate });
    }
}

impl Program {
    pub fn new(initial_memory: &Vec<i32>) -> Program {
        Program {
            ip: 0,
            memory: initial_memory.clone(),
            halted: false,
            jumped: false,
            _input: None,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn read(&self, addr: usize) -> i32 {
        self.memory[addr]
    }

    pub fn set_input(&mut self, input: i32) {
        self._input = Some(input);
    }

    fn raw_param(&self, offset: usize) -> i32 {
        self.read(self.ip + offset)
    }

    fn write(&mut self, addr: usize, value: i32) {
        self.memory[addr] = value;
    }

    fn read_param(&self, param: &Param) -> i32 {
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
            let immediate = (param_modifiers / 10i32.pow(i as u32 + 1)) % 2 == 1;

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
        match self._input {
            Some(input) => self.write(dst, input),
            None => panic!("No input available"),
        }
    }

    fn output(&self, params: &Vec<Param>) {
        let val = self.read_param(&params[0]);
        println!("{}", val);
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

        self.write(dst, (val0 < val1) as i32);
    }

    fn eq(&mut self, params: &Vec<Param>) {
        let val0 = self.read_param(&params[0]);
        let val1 = self.read_param(&params[1]);
        let dst = params[2].value as usize;

        self.write(dst, (val0 == val1) as i32);
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}
