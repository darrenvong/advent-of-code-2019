mod computer;
pub use computer::Computer;

mod parser;

pub enum Operation {
    Add {
        input1: usize,
        input2: usize,
        output: usize,
        length: usize
    },
    Multiply {
        input1: usize,
        input2: usize,
        output: usize,
        length: usize
    },
    Stop { length: usize }
}
