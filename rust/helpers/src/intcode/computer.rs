use std::fs::File;
use std::io::{BufReader, Read};

use super::{Operation, Input};
use super::parser::Parser;

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
        let mut head_pos = 0;

        while head_pos < self.registers.len() {
            let parser = Parser::new(&self);
            let operation = parser.parse(head_pos);

            if let Operation::Stop { .. } = operation {
                break;
            } else {
                let length = self.perform_op(operation);
                head_pos += length;
            }
        }
    }

    fn perform_op(&mut self, operation: Operation) -> usize {
        match operation {
            Operation::Add { input1, input2, output, length } => {
                let left = match input1 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                let right = match input2 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                self.registers[output] = left + right;
                length
            }
            Operation::Multiply { input1, input2, output, length } => {
                let left = match input1 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                let right = match input2 {
                    Input::Position(i) => { self.registers[i] },
                    Input::Immediate(v) => v
                };
                self.registers[output] = left * right;
                length
            },
            Operation::Save { input, length } => {
                match input {
                    Input::Position(i) => self.registers[i] = self.input,
                    _ => panic!("Immediate mode not supported for this command")
                };
                length
            },
            Operation::Print { output, length } => {
                println!("Opcode 4 output: {}", self.registers[output]);
                length
            },
            Operation::Stop { length } => { length }
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
