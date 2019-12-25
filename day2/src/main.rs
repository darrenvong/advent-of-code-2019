use std::io::prelude::*;
use std::error::Error;
use helpers;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = helpers::read_puzzle_input("day2/input.txt");
    
    let mut program_code = String::new();
    reader.read_to_string(&mut program_code)?;

    let mut program_code = program_code.trim()
        .split(",")
        .map(|code| {
            code.parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>();
    
    program_code[1] = 12;
    program_code[2] = 2;
    // for (head_pos, code) in program_code.iter_mut().enumerate().step_by(4) {
    //     let mut first_addr = program_code[head_pos + 1];
    // }
    
    println!("{:?}", program_code);
    let part1 = 0;

    let part2 = 0;

    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    println!("The value at position 0 is: {}", part1);

    println!("\n{} Part 2 {}", marker, marker);
    println!("Part 2: {}", part2);

    Ok(())
}
