use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use helpers;
use helpers::intcode::{self, Input};

fn find_input(target: i32, reader: &mut BufReader<File>) -> Input {
    for noun in 0..100 {
        for verb in 0..100 {
            let noun: i32 = noun.try_into().unwrap();
            let verb: i32 = verb.try_into().unwrap();

            let mut computer = intcode::Computer::new(reader);
            computer.run_program(Input { noun, verb });

            if computer.get_register_value(0) == target {
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
    
    let mut computer = intcode::Computer::new(&mut reader);
    computer.run_program(Input { noun: 12, verb: 2});
    let part1 = computer.get_register_value(0);

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
