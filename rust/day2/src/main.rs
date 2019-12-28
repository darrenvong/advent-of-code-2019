use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;

use helpers;
use helpers::intcode;

struct Input {
    noun: i32,
    verb: i32
}

fn find_input(target: i32, reader: &mut BufReader<File>) -> Input {
    for noun in 0..100 {
        for verb in 0..100 {
            let noun: i32 = noun.try_into().unwrap();
            let verb: i32 = verb.try_into().unwrap();

            let mut computer = intcode::Computer::new(reader);
            computer.set_register_value(1, noun);
            computer.set_register_value(2, verb);
            computer.run_program();

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

fn main() {
    let mut reader = helpers::read_puzzle_input("day2/input.txt");
    
    let mut computer = intcode::Computer::new(&mut reader);

    computer.set_register_value(1, 12);
    computer.set_register_value(2, 2);
    computer.run_program();
    let part1 = computer.get_register_value(0);

    helpers::reset_reader(&mut reader);

    let Input { noun, verb } = find_input(19690720, &mut reader);

    let part2 = 100 * noun + verb;

    helpers::print_answers(
        "The value at position 0 is: {part1}",
        "100 * noun + verb: {part2}",
        helpers::Answer {
            part1: Box::new(part1),
            part2: Box::new(part2)
        }
    );
}
