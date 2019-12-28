extern crate strfmt;

use std::collections::HashMap;
use std::io::{BufReader, Seek, SeekFrom};
use std::fmt::Display;
use std::fs::File;
use strfmt::strfmt;

pub mod intcode;

pub fn read_puzzle_input(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    reader
}

/// Reset the buffer reader position so its content can be
/// read for another time.
pub fn reset_reader(reader: &mut BufReader<File>) {
    reader.seek(SeekFrom::Start(0)).unwrap();
}

pub struct Answer {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>
}

pub fn print_answers(part1_fmt: &str, part2_fmt: &str, answer: Answer) {
    let marker = "-".repeat(15);
    let Answer { part1, part2 } = answer;

    let mut vars = HashMap::new();
    vars.insert("part1".to_string(), part1);
    vars.insert("part2".to_string(), part2);

    println!("{} Part 1 {}", marker, marker);
    println!("{}", strfmt(part1_fmt, &vars).unwrap());

    println!("\n{} Part 2 {}", marker, marker);
    println!("{}", strfmt(part2_fmt, &vars).unwrap());
}
