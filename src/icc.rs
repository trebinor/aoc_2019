#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(PartialEq, PartialOrd)]
enum Operation {
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

#[derive(Clone)]
pub struct IntCodeComputer {
    pub program: Vec<i32>,
    pub pc: usize,
    pub input0: i32,
    pub input1: i32,
    pub input0_read: bool,
    pub output: i32,
    pub terminated: bool,
}

impl IntCodeComputer {
    pub fn execute(&mut self) -> i32 {
        loop {
            let s = format!("{:0>5}", self.program[self.pc].to_string());
            let mut c = s.chars();
            //println!("{:?}", c);
            let _p2 = c.next();
            let p1 = match c.next().unwrap() {
                '0' => ParameterMode::Position,
                '1' => ParameterMode::Immediate,
                _ => unreachable!(),
            };
            let p0 = match c.next().unwrap() {
                '0' => ParameterMode::Position,
                '1' => ParameterMode::Immediate,
                _ => unreachable!(),
            };
            let operation = c.take(2).collect::<String>().parse::<i32>().unwrap();
            //println!("operation={}", operation);
            match operation {
                1 => self.add(p0, p1),
                2 => self.mul(p0, p1),
                3 => self.store(),
                //4 => output.push_str(&self.show(p0)),
                4 => {
                    self.output = self.show(p0).parse::<i32>().unwrap();
                    break;
                }
                5 => self.conditional(p0, p1, Operation::JumpIfTrue),
                6 => self.conditional(p0, p1, Operation::JumpIfFalse),
                7 => self.conditional(p0, p1, Operation::LessThan),
                8 => self.conditional(p0, p1, Operation::Equals),
                99 => {
                    self.terminated = true;
                    break;
                }
                _ => panic!(),
            }
        }
        self.output
    }

    fn immediate(&self, offset: usize) -> i32 {
        self.program[self.pc + offset]
    }

    fn position(&self, offset: usize) -> i32 {
        self.program[self.immediate(offset) as usize]
    }

    fn add(&mut self, p0: ParameterMode, p1: ParameterMode) {
        let a0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
        };
        let a1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
        };
        let output_addr = self.immediate(3);
        self.program[output_addr as usize] = a0 + a1;
        self.pc += 4;
    }

    fn mul(&mut self, p0: ParameterMode, p1: ParameterMode) {
        let a0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
        };
        let a1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
        };
        let output_addr = self.immediate(3);
        self.program[output_addr as usize] = a0 * a1;
        self.pc += 4;
    }

    fn store(&mut self) {
        let output_addr = self.immediate(1);
        self.program[output_addr as usize] = if self.input0_read {
            self.input1
        } else {
            self.input0_read = true;
            self.input0
        };
        self.pc += 2;
    }

    fn show(&mut self, p0: ParameterMode) -> String {
        let s0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
        };
        self.pc += 2;
        s0.to_string()
    }

    fn conditional(&mut self, p0: ParameterMode, p1: ParameterMode, o: Operation) {
        let o0 = match p0 {
            ParameterMode::Position => self.position(1),
            ParameterMode::Immediate => self.immediate(1),
        };
        let o1 = match p1 {
            ParameterMode::Position => self.position(2),
            ParameterMode::Immediate => self.immediate(2),
        };
        let output_addr = self.immediate(3);
        if o == Operation::JumpIfTrue {
            if o0 != 0 {
                self.pc = o1 as usize;
            } else {
                self.pc += 3;
            }
        } else if o == Operation::JumpIfFalse {
            if o0 == 0 {
                self.pc = o1 as usize;
            } else {
                self.pc += 3;
            }
        } else if o == Operation::LessThan {
            self.program[output_addr as usize] = if o0 < o1 { 1 } else { 0 };
            self.pc += 4;
        } else if o == Operation::Equals {
            self.program[output_addr as usize] = if o0 == o1 { 1 } else { 0 };
            self.pc += 4;
        }
    }
}
