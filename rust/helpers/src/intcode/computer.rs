use std::fs::File;
use std::io::{BufReader, Read};

use super::Operation;
use super::parser::Parser;

pub struct Computer {
    registers: Vec<i32>
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
                self.registers[output] = self.registers[input1] + self.registers[input2];
                length
            },
            Operation::Multiply { input1, input2, output, length } => {
                self.registers[output] = self.registers[input1] * self.registers[input2];
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
}
