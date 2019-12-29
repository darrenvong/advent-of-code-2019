use std::fs::File;
use std::io::{BufReader, Read};

use super::{Operation, Input};
use super::parser::Parser;

enum PointerMove {
    Increment(usize),
    Jump(usize)
}

pub struct Computer {
    registers: Vec<i32>,
    input: i32
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
            registers,
            input: -1 // Doesn't matter what this is in day 2's problem...
        }
    }

    pub fn with_input(reader: &mut BufReader<File>, input: i32) -> Computer {
        let mut computer = Computer::new(reader);
        computer.input = input;
        computer
    }

    pub fn run_program(&mut self) {
        let mut head_pos: usize = 0;

        while head_pos < self.registers.len() {
            let parser = Parser::new(&self);
            let operation = parser.parse(head_pos);

            if let Operation::Stop { .. } = operation {
                break;
            } else {
                let length = self.perform_op(operation);
                match length {
                    PointerMove::Increment(size) => head_pos += size,
                    PointerMove::Jump(index) => head_pos = index
                }
            }
        }
    }

    fn perform_op(&mut self, operation: Operation) -> PointerMove {
        match operation {
            Operation::Add { input1, input2, output, length } |
            Operation::Multiply { input1, input2, output, length } |
            Operation::LessThan { input1, input2, output, length } |
            Operation::Equal { input1, input2, output, length } => {
                let first = match input1 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                let second = match input2 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };

                match operation {
                    Operation::Add {..} => self.registers[output] = first + second,
                    Operation::Multiply {..} => self.registers[output] = first * second,
                    Operation::LessThan {..} => {
                        self.registers[output] = if first < second { 1 } else { 0 };
                    },
                    Operation::Equal {..} => {
                        self.registers[output] = if first == second { 1 } else { 0 };
                    },
                    _ => panic!("Impossible case")
                }
                PointerMove::Increment(length)
            },
            Operation::Save { input, length } => {
                match input {
                    Input::Position(i) => self.registers[i] = self.input,
                    _ => panic!("Immediate mode not supported for this command")
                };
                PointerMove::Increment(length)
            },
            Operation::Print { output, length } => {
                println!("Opcode 4 output: {}", self.registers[output]);
                PointerMove::Increment(length)
            },
            Operation::JumpIfTrue { input1, input2, length } |
            Operation::JumpIfFalse { input1, input2, length } => {
                let first = match input1 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                let second = match input2 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };

                match operation {
                    Operation::JumpIfTrue {..} => {
                        if first != 0 {
                            PointerMove::Jump(second as usize)
                        } else { PointerMove::Increment(length) }
                    },
                    Operation::JumpIfFalse {..} => {
                        if first == 0 {
                            PointerMove::Jump(second as usize)
                        } else { PointerMove:: Increment(length) }
                    },
                    _ => panic!("Impossible case")
                }
            },
            Operation::Stop { length } => { PointerMove::Increment(length) }
        }
    }

    pub fn get_register_value(&self, index: usize) -> i32 {
        self.registers[index]
    }

    pub fn set_register_value(&mut self, index: usize, value: i32) {
        self.registers[index] = value;
    }

    pub fn get_input(&self) -> i32 {
        self.input
    }
}
