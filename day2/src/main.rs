use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use helpers;

fn translate_to_program_code(reader: &mut BufReader<File>) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut program_code = String::new();
    reader.read_to_string(&mut program_code)?;

    let program_code: Vec<usize> = program_code.trim()
        .split(",")
        .map(|code| {
            code.parse::<usize>().unwrap()
        })
        .collect();

    Ok(program_code)
}

fn run_intcode_program(noun: usize, verb: usize, program_code: &mut Vec<usize>) {
    program_code[1] = noun;
    program_code[2] = verb;

    let mut head_pos = 0;

    while head_pos < program_code.len() {
        let opcode = program_code[head_pos];
        match opcode {
            1 => {
                let input1_pos = program_code[head_pos + 1];
                let input2_pos = program_code[head_pos + 2];
                let output_pos = program_code[head_pos + 3];

                program_code[output_pos] = program_code[input1_pos] + program_code[input2_pos];
            },
            2 => {
                let input1_pos = program_code[head_pos + 1];
                let input2_pos = program_code[head_pos + 2];
                let output_pos = program_code[head_pos + 3];

                program_code[output_pos] = program_code[input1_pos] * program_code[input2_pos];
            },
            99 => break,
            _ => panic!("Invalid operation encountered!")
        }

        head_pos += 4;
    }
}

struct Input {
    noun: usize,
    verb: usize
}

fn find_input(target: usize, reader: &mut BufReader<File>) -> Input {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program_code = translate_to_program_code(reader).unwrap();
            run_intcode_program(noun, verb, &mut program_code);

            if program_code[0] == target {
                return Input {
                    noun,
                    verb
                };
            } else {
                helpers::reset_reader(reader);
            }
        }
    }

    panic!("Noun and verb for target not found...");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = helpers::read_puzzle_input("day2/input.txt");
    
    let mut program_code = translate_to_program_code(&mut reader)?;
    
    run_intcode_program(12, 2, &mut program_code);
    let part1 = program_code[0];

    helpers::reset_reader(&mut reader);

    let Input { noun, verb } = find_input(19690720, &mut reader);

    let part2 = 100 * noun + verb;

    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    println!("The value at position 0 is: {}", part1);

    println!("\n{} Part 2 {}", marker, marker);
    println!("100 * noun + verb: {}", part2);

    Ok(())
}
