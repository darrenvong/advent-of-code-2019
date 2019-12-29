use std::iter::Rev;
use std::str::Chars;

use super::{Computer, Input, Operation};

pub struct Parser<'a> {
    computer: &'a Computer
}

impl<'a> Parser<'a> {
    pub fn new(computer: &Computer) -> Parser {
        Parser { computer }
    }

    pub fn parse(&self, head_pos: usize) -> Operation {
        let raw_opcode_mode = self.computer.get_register_value(head_pos);
        let opcode_mode = raw_opcode_mode.to_string();
        let mut opcode_mode_it = opcode_mode.chars().rev();

        let opcode = self.get_opcode(&mut opcode_mode_it).unwrap();
        match opcode {
            1 | 2 => {
                let input1 = self.create_input_with_mode(head_pos + 1, opcode_mode_it.next());
                let input2 = self.create_input_with_mode(head_pos + 2, opcode_mode_it.next());
                let output = self.computer.get_register_value(head_pos + 3) as usize;
                let length = 4;

                return if opcode == 1 {
                    Operation::Add { input1, input2, output, length }
                } else {
                    Operation::Multiply { input1, input2, output, length }
                };
            },
            3 => {
                let value = self.computer.get_register_value(head_pos + 1) as usize;
                Operation::Save { input: Input::Position(value), length: 2 }
            },
            4 => {
                let output = self.computer.get_register_value(head_pos + 1) as usize;
                Operation::Print { output, length: 2 }
            },
            99 => Operation::Stop { length: 1 },
            _ => panic!("Invalid operation encountered!")
        }
    }

    fn create_input_with_mode(&self, pos: usize, mode_value: Option<char>) -> Input {
        let value = self.computer.get_register_value(pos);
        let mode = mode_value.unwrap_or_else(|| '0').to_digit(10).unwrap();
        if mode == 0 { Input::Position(value as usize) } else { Input::Immediate(value) }
    }

    fn get_opcode(&self, opcode_it: &mut Rev<Chars>) -> Option<u32> {
        let first = opcode_it.next().unwrap_or_else(|| '0').to_digit(10)?;
        let second = opcode_it.next().unwrap_or_else(|| '0').to_digit(10)?;
        Some(first + 10 * second)
    }
}
