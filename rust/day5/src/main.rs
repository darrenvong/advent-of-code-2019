use helpers;
use helpers::intcode;

fn main() {
    let mut reader = helpers::read_puzzle_input("day5/input.txt");
    
    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    let mut computer = intcode::Computer::with_input(&mut reader, 1);
    computer.run_program();

    helpers::reset_reader(&mut reader);

    println!("\n{} Part 2 {}", marker, marker);
    let mut computer2 = intcode::Computer::with_input(&mut reader, 5);
    computer2.run_program();
}
