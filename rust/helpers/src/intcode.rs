use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Input {
    pub noun: i32,
    pub verb: i32
}

pub struct Computer {
    registers: Vec<i32>
}

enum Operation {
    Add {
        input1: usize,
        input2: usize,
        output: usize
    },
    Multiply {
        input1: usize,
        input2: usize,
        output: usize
    }
}

impl Computer {
    pub fn new(reader: &mut BufReader<File>) -> Computer {
        let mut program_code = String::new();
        reader.read_to_string(&mut program_code).unwrap();

        let registers: Vec<i32> = program_code.trim()
            .split(",")
            .map(|code| {
                code.parse::<i32>().unwrap()
            })
            .collect();
        
        Computer {
            registers
        }
    }

    pub fn run_program(&mut self, input: Input) {
        self.registers[1] = input.noun;
        self.registers[2] = input.verb;

        let mut head_pos = 0;

        while head_pos < self.registers.len() {
            let opcode = self.registers[head_pos];
            match opcode {
                1 | 2 => {
                    let input1: usize = self.registers[head_pos + 1].try_into().unwrap();
                    let input2: usize = self.registers[head_pos + 2].try_into().unwrap();
                    let output: usize = self.registers[head_pos + 3].try_into().unwrap();

                    let operation = if opcode == 1 {
                        Operation::Add { input1, input2, output }
                    } else {
                        Operation::Multiply { input1, input2, output }
                    };
                    self.perform_op(operation);
                    head_pos += 4;
                },
                99 => break,
                _ => panic!("Invalid operation encountered!")
            }
        }
    }

    fn perform_op(&mut self, operation: Operation) {
        match operation {
            Operation::Add { input1, input2, output } => {
                self.registers[output] = self.registers[input1] + self.registers[input2]
            },
            Operation::Multiply { input1, input2, output } => {
                self.registers[output] = self.registers[input1] * self.registers[input2]
            }
        }
    }

    pub fn get_register_value(&self, index: usize) -> i32 {
        self.registers[index]
    }
}
