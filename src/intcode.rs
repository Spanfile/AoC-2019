use std::collections::VecDeque;

#[derive(Debug)]
pub struct IntcodeVM {
    memory: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Position(usize),
    Immediate(usize),
}

#[derive(Debug, Copy, Clone)]
enum OpcodeOutput {
    None,
    Halt,
    Output(i64),
    Jump(usize),
}

fn ones(val: i64) -> i64 {
    val % 10
}

fn tens(val: i64) -> i64 {
    val % 100 / 10
}

fn hundreds(val: i64) -> i64 {
    val % 1000 / 100
}

fn thousands(val: i64) -> i64 {
    val % 10_000 / 1000
}

fn tens_thousands(val: i64) -> i64 {
    val % 100_000 / 10_000
}

impl Opcode {
    fn from_memory(index: usize, memory: &[i64]) -> Self {
        let opcode_value = memory[index];
        let opcode = (tens(opcode_value) * 10) + ones(opcode_value);

        match opcode {
            1 => Self::Add(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
                Parameter::new(tens_thousands(opcode_value), memory[index + 3]),
            ),
            2 => Self::Mul(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
                Parameter::new(tens_thousands(opcode_value), memory[index + 3]),
            ),
            3 => Self::Input(Parameter::new(hundreds(opcode_value), memory[index + 1])),
            4 => Self::Output(Parameter::new(hundreds(opcode_value), memory[index + 1])),
            5 => Self::JumpIfTrue(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
            ),
            6 => Self::JumpIfFalse(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
            ),
            7 => Self::LessThan(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
                Parameter::new(tens_thousands(opcode_value), memory[index + 3]),
            ),
            8 => Self::Equals(
                Parameter::new(hundreds(opcode_value), memory[index + 1]),
                Parameter::new(thousands(opcode_value), memory[index + 2]),
                Parameter::new(tens_thousands(opcode_value), memory[index + 3]),
            ),
            99 => Self::Halt,
            _ => panic!(
                "something went terribly wrong (opcode {} at {})",
                opcode, index
            ),
        }
    }

    fn execute(self, memory: &mut [i64], input: &mut VecDeque<i64>) -> OpcodeOutput {
        match self {
            Self::Add(p1, p2, dest) => {
                memory[dest.position()] = p1.evaluate(memory) + p2.evaluate(memory);
                OpcodeOutput::None
            }
            Self::Mul(p1, p2, dest) => {
                memory[dest.position()] = p1.evaluate(memory) * p2.evaluate(memory);
                OpcodeOutput::None
            }
            Self::Input(dest) => {
                memory[dest.position()] = input.pop_front().unwrap();
                OpcodeOutput::None
            }
            Self::Output(dest) => OpcodeOutput::Output(dest.evaluate(memory)),
            Self::JumpIfTrue(p1, p2) => {
                if p1.evaluate(memory) != 0 {
                    OpcodeOutput::Jump(p2.evaluate(memory) as usize)
                } else {
                    OpcodeOutput::None
                }
            }
            Self::JumpIfFalse(p1, p2) => {
                if p1.evaluate(memory) == 0 {
                    OpcodeOutput::Jump(p2.evaluate(memory) as usize)
                } else {
                    OpcodeOutput::None
                }
            }
            Self::LessThan(p1, p2, dest) => {
                memory[dest.position()] = if p1.evaluate(memory) < p2.evaluate(memory) {
                    1
                } else {
                    0
                };
                OpcodeOutput::None
            }
            Self::Equals(p1, p2, dest) => {
                memory[dest.position()] = if p1.evaluate(memory) == p2.evaluate(memory) {
                    1
                } else {
                    0
                };
                OpcodeOutput::None
            }
            Self::Halt => OpcodeOutput::Halt,
        }
    }

    fn len(self) -> usize {
        // length of opcode + length of parameters
        1 + match self {
            Self::Add(..) | Self::Mul(..) | Self::LessThan(..) | Self::Equals(..) => 3,
            Self::JumpIfTrue(..) | Self::JumpIfFalse(..) => 2,
            Self::Input(..) | Self::Output(..) => 1,
            Self::Halt => 0,
        }
    }
}

impl Parameter {
    fn new(mode: i64, value: i64) -> Self {
        match mode {
            0 => Self::Position(value as usize),
            1 => Self::Immediate(value as usize),
            _ => panic!(),
        }
    }

    fn evaluate(self, memory: &[i64]) -> i64 {
        match self {
            Self::Position(value) => memory[value],
            Self::Immediate(value) => value as i64,
        }
    }

    fn position(self) -> usize {
        match self {
            Self::Position(value) => value,
            _ => panic!(),
        }
    }
}

impl IntcodeVM {
    pub fn new(memory: Vec<i64>) -> IntcodeVM {
        IntcodeVM {
            memory,
            ip: 0,
            input: VecDeque::new(),
        }
    }

    pub fn input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn get_next_output(&mut self) -> Option<i64> {
        loop {
            let opcode = Opcode::from_memory(self.ip, &self.memory);

            match opcode.execute(&mut self.memory, &mut self.input) {
                OpcodeOutput::Halt => break None,
                OpcodeOutput::Output(value) => {
                    self.ip += opcode.len();
                    break Some(value);
                }
                OpcodeOutput::Jump(ip) => {
                    self.ip = ip;
                    continue;
                }
                _ => (),
            }

            self.ip += opcode.len();
        }
    }
}
