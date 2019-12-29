use helpers;
use helpers::intcode;

fn main() {
    let mut reader = helpers::read_puzzle_input("day5/input.txt");
    
    let marker = "-".repeat(15);
    println!("{} Part 1 {}", marker, marker);
    let mut computer = intcode::Computer::with_input(&mut reader, 1);
    computer.run_program();

    helpers::reset_reader(&mut reader);

    let mut computer2 = intcode::Computer::with_input(&mut reader, 5);

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
