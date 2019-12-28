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
        let mass = mass.unwrap().parse::<isize>().unwrap();
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

    helpers::print_answers(
        "The sum of fuel requirements: {part1}",
        "The sum of fuel requirements: {part2}",
        helpers::Answer {
            part1: Box::new(part1),
            part2: Box::new(part2)
        }
    );
}
