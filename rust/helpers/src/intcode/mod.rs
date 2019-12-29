mod computer;
pub use computer::Computer;
mod parser;

#[derive(Copy, Clone)]
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
    Save { input: Input, length: usize },
    Print { output: usize, length: usize },
    JumpIfTrue {
        input1: Input,
        input2: Input,
        length: usize
    },
    JumpIfFalse {
        input1: Input,
        input2: Input,
        length: usize
    },
    LessThan {
        input1: Input,
        input2: Input,
        output: usize,
        length: usize
    },
    Equal {
        input1: Input,
        input2: Input,
        output: usize,
        length: usize
    },
    Stop { length: usize }
}

#[derive(Copy, Clone)]
pub enum Input {
    Position(usize),
    Immediate(i32)
}
