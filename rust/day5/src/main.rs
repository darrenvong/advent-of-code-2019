use std::fs::File;
use std::io::BufReader;

use helpers;
use helpers::intcode;

fn main() {
    let mut reader = helpers::read_puzzle_input("day5/input.txt");
    
    let computer = intcode::Computer::new(&mut reader);

    println!("{}", "02".parse::<i32>().unwrap());

    // let part1 = 0;

    // let part2 = 0;

    // helpers::print_answers(
    //     "Part 1: {part1}",
    //     "Part 2: {part2}",
    //     helpers::Answer {
    //         part1: Box::new(part1),
    //         part2: Box::new(part2)
    //     }
    // );
}
