use std::collections::VecDeque;

#[derive(Debug)]
pub struct IntcodeVM {
    memory: Memory,
    ip: usize,
    input: VecDeque<i64>,
    rel_base_offset: usize,
}

#[derive(Debug)]
struct Memory {
    memory: Vec<i64>,
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
    RelativeBaseOffset(Parameter),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Position(usize),
    Immediate(usize),
    Relative(usize),
}

#[derive(Debug, Copy, Clone)]
enum OpcodeOutput {
    None,
    Halt,
    Output(i64),
    Jump(usize),
    NewBaseOffset(usize),
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
    fn from_memory(index: usize, memory: &mut Memory) -> Self {
        let opcode_value = memory.get(index);
        let opcode = (tens(opcode_value) * 10) + ones(opcode_value);

        match opcode {
            1 => Self::Add(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
                Parameter::new(tens_thousands(opcode_value), memory.get(index + 3)),
            ),
            2 => Self::Mul(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
                Parameter::new(tens_thousands(opcode_value), memory.get(index + 3)),
            ),
            3 => Self::Input(Parameter::new(
                hundreds(opcode_value),
                memory.get(index + 1),
            )),
            4 => Self::Output(Parameter::new(
                hundreds(opcode_value),
                memory.get(index + 1),
            )),
            5 => Self::JumpIfTrue(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
            ),
            6 => Self::JumpIfFalse(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
            ),
            7 => Self::LessThan(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
                Parameter::new(tens_thousands(opcode_value), memory.get(index + 3)),
            ),
            8 => Self::Equals(
                Parameter::new(hundreds(opcode_value), memory.get(index + 1)),
                Parameter::new(thousands(opcode_value), memory.get(index + 2)),
                Parameter::new(tens_thousands(opcode_value), memory.get(index + 3)),
            ),
            9 => Self::RelativeBaseOffset(Parameter::new(
                hundreds(opcode_value),
                memory.get(index + 1),
            )),
            99 => Self::Halt,
            _ => panic!(
                "something went terribly wrong (opcode {} at {})",
                opcode, index
            ),
        }
    }

    fn execute(
        self,
        memory: &mut Memory,
        input: &mut VecDeque<i64>,
        rel_base_offset: usize,
    ) -> OpcodeOutput {
        match self {
            Self::Add(p1, p2, dest) => {
                let value =
                    p1.evaluate(memory, rel_base_offset) + p2.evaluate(memory, rel_base_offset);
                memory.set(dest.position(rel_base_offset), value);

                OpcodeOutput::None
            }
            Self::Mul(p1, p2, dest) => {
                let value =
                    p1.evaluate(memory, rel_base_offset) * p2.evaluate(memory, rel_base_offset);
                memory.set(dest.position(rel_base_offset), value);

                OpcodeOutput::None
            }
            Self::Input(dest) => {
                memory.set(dest.position(rel_base_offset), input.pop_front().unwrap());
                OpcodeOutput::None
            }
            Self::Output(dest) => OpcodeOutput::Output(dest.evaluate(memory, rel_base_offset)),
            Self::JumpIfTrue(p1, p2) => {
                if p1.evaluate(memory, rel_base_offset) != 0 {
                    OpcodeOutput::Jump(p2.evaluate(memory, rel_base_offset) as usize)
                } else {
                    OpcodeOutput::None
                }
            }
            Self::JumpIfFalse(p1, p2) => {
                if p1.evaluate(memory, rel_base_offset) == 0 {
                    OpcodeOutput::Jump(p2.evaluate(memory, rel_base_offset) as usize)
                } else {
                    OpcodeOutput::None
                }
            }
            Self::LessThan(p1, p2, dest) => {
                let value = if p1.evaluate(memory, rel_base_offset)
                    < p2.evaluate(memory, rel_base_offset)
                {
                    1
                } else {
                    0
                };
                memory.set(dest.position(rel_base_offset), value);

                OpcodeOutput::None
            }
            Self::Equals(p1, p2, dest) => {
                let value = if p1.evaluate(memory, rel_base_offset)
                    == p2.evaluate(memory, rel_base_offset)
                {
                    1
                } else {
                    0
                };
                memory.set(dest.position(rel_base_offset), value);

                OpcodeOutput::None
            }
            Self::RelativeBaseOffset(p1) => OpcodeOutput::NewBaseOffset(
                rel_base_offset + p1.evaluate(memory, rel_base_offset) as usize,
            ),
            Self::Halt => OpcodeOutput::Halt,
        }
    }

    fn len(self) -> usize {
        // length of opcode + length of parameters
        1 + match self {
            Self::Add(..) | Self::Mul(..) | Self::LessThan(..) | Self::Equals(..) => 3,
            Self::JumpIfTrue(..) | Self::JumpIfFalse(..) => 2,
            Self::Input(..) | Self::Output(..) | Self::RelativeBaseOffset(..) => 1,
            Self::Halt => 0,
        }
    }
}

impl Parameter {
    fn new(mode: i64, value: i64) -> Self {
        match mode {
            0 => Self::Position(value as usize),
            1 => Self::Immediate(value as usize),
            2 => Self::Relative(value as usize),
            _ => panic!(),
        }
    }

    fn evaluate(self, memory: &mut Memory, rel_base_offset: usize) -> i64 {
        match self {
            Self::Position(value) => memory.get(value),
            Self::Immediate(value) => value as i64,
            Self::Relative(value) => memory.get(value + rel_base_offset),
        }
    }

    fn position(self, rel_base_offset: usize) -> usize {
        match self {
            Self::Position(value) => value,
            Self::Relative(value) => value + rel_base_offset,
            _ => panic!(),
        }
    }
}

impl Memory {
    fn expand_to(&mut self, capacity: usize) {
        let mut extension = vec![0; capacity - self.memory.len()];
        self.memory.reserve(extension.len());
        self.memory.append(&mut extension);
    }

    fn get(&mut self, index: usize) -> i64 {
        if self.memory.len() <= index {
            self.expand_to(index + 1);
        }

        self.memory[index]
    }

    fn set(&mut self, index: usize, value: i64) {
        if self.memory.len() <= index {
            self.expand_to(index + 1);
        }

        self.memory[index] = value;
    }
}

impl IntcodeVM {
    pub fn new(memory: Vec<i64>) -> IntcodeVM {
        IntcodeVM {
            memory: Memory { memory },
            ip: 0,
            input: VecDeque::new(),
            rel_base_offset: 0,
        }
    }

    pub fn input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn get_next_output(&mut self) -> Option<i64> {
        loop {
            let opcode = Opcode::from_memory(self.ip, &mut self.memory);

            match opcode.execute(&mut self.memory, &mut self.input, self.rel_base_offset) {
                OpcodeOutput::Halt => break None,
                OpcodeOutput::Output(value) => {
                    self.ip += opcode.len();
                    break Some(value);
                }
                OpcodeOutput::Jump(ip) => {
                    self.ip = ip;
                    continue;
                }
                OpcodeOutput::NewBaseOffset(new_base) => self.rel_base_offset = new_base,
                OpcodeOutput::None => (),
            }

            self.ip += opcode.len();
        }
    }
}
