mod computer;
pub use computer::Computer;
mod parser;

pub enum Operation {
    Add {
        input1: Input,
        input2: Input,
        output: usize,
        length: usize
    },
    Multiply {
        input1: Input,
        input2: Input,
        output: usize,
        length: usize
    },
    Stop { length: usize }
}

pub enum Input {
    Position(usize),
    Immediate(i32)
}
