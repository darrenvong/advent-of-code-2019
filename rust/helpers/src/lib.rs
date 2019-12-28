use std::io::{BufReader, Seek, SeekFrom};
use std::fs::File;

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
