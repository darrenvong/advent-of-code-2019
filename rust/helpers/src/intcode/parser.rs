use std::convert::TryInto;

use super::Computer;
use super::Operation;

pub struct Parser<'a> {
    computer: &'a Computer
}

impl<'a> Parser<'a> {
    pub fn new(computer: &Computer) -> Parser {
        Parser { computer }
    }

    pub fn parse(&self, head_pos: usize) -> Operation {
        let opcode = self.computer.get_register_value(head_pos);
        match opcode {
            1 | 2 => {
                let input1: usize = self.computer.get_register_value(head_pos + 1).try_into().unwrap();
                let input2: usize = self.computer.get_register_value(head_pos + 2).try_into().unwrap();
                let output: usize = self.computer.get_register_value(head_pos + 3).try_into().unwrap();
                let length = 4;

                return if opcode == 1 {
                    Operation::Add { input1, input2, output, length }
                } else {
                    Operation::Multiply { input1, input2, output, length }
                };
            },
            99 => Operation::Stop { length: 1 },
            _ => panic!("Invalid operation encountered!")
        }
    }
}
