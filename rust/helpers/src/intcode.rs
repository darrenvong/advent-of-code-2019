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
                1 => {
                    let input1_pos: usize = self.registers[head_pos + 1].try_into().unwrap();
                    let input2_pos: usize = self.registers[head_pos + 2].try_into().unwrap();
                    let output_pos: usize = self.registers[head_pos + 3].try_into().unwrap();

                    self.registers[output_pos] = self.registers[input1_pos] + self.registers[input2_pos];
                },
                2 => {
                    let input1_pos: usize = self.registers[head_pos + 1].try_into().unwrap();
                    let input2_pos: usize = self.registers[head_pos + 2].try_into().unwrap();
                    let output_pos: usize = self.registers[head_pos + 3].try_into().unwrap();

                    self.registers[output_pos] = self.registers[input1_pos] * self.registers[input2_pos];
                },
                99 => break,
                _ => panic!("Invalid operation encountered!")
            }

            head_pos += 4;
        }
    }

    pub fn get_register_value(&self, index: usize) -> i32 {
        self.registers[index]
    }
}
