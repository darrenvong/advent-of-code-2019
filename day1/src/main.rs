use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let part1: isize = reader.lines()
        .map(|mass| {
            let mass = mass.unwrap();
            let mass = mass.parse::<isize>().unwrap();
            mass / 3 - 2
        })
        .sum();
    
    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    println!("The sum of fuel requirements: {}", part1);

    println!("\n{} Part 2 {}", marker, marker);
}
