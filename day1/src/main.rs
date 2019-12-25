use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use helpers;

fn calculate_fuel_requirements<F>(
    reader: &mut BufReader<File>,
    get_fuel_requirement: F) -> isize
    where F: Fn(Result<String, std::io::Error>) -> isize
{
    reader.lines()
        .map(get_fuel_requirement)
        .sum()
}

fn main() {
    let mut reader = helpers::read_puzzle_input("day1/input.txt");

    let part1 = calculate_fuel_requirements(&mut reader, |mass| {
        let mass = mass.unwrap();
        let mass = mass.parse::<isize>().unwrap();
        mass / 3 - 2
    });

    helpers::reset_reader(&mut reader);

    let part2 = calculate_fuel_requirements(&mut reader, |mass| {
        let mut mass = mass.unwrap().parse::<isize>().unwrap();
        let mut total_mass = 0;

        while mass >= 0 {
            mass = mass / 3 - 2;
            total_mass = if mass > 0 { total_mass + mass } else { total_mass };
        }
        total_mass
    });
    
    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    println!("The sum of fuel requirements: {}", part1);

    println!("\n{} Part 2 {}", marker, marker);
    println!("The sum of fuel requirements: {}", part2);
}
