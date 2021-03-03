use std::error::Error;
use std::{
    fmt::{Display, Formatter},
    marker::Sync,
};
use std::{fs::File, io::Read};

pub enum Part {
    One,
    Two,
}

#[derive(Debug)]
pub(crate) enum SolveErrorType {
    ReadError,
    ValidationError,
}
#[derive(Debug)]
pub struct SolveError {
    etype: SolveErrorType,
    message: String,
}

impl Display for SolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error type: {:?}\nReason: {}", self.etype, self.message)
    }
}

impl Error for SolveError {}

pub trait Solver: Sync {
    fn validate(&self, input: &[u8]) -> bool;
    fn solve_part_one(&self, data: Vec<u8>) -> String;
    fn solve_part_two(&self, data: Vec<u8>) -> String;
    fn solve(&self, mut f: File, part: Part) -> Result<String, SolveError> {
        let mut buf = Vec::with_capacity(1000);
        match f.read_to_end(&mut buf) {
            Ok(_) => {}
            Err(e) => eprint!("{}", e),
        };
        let solution = match part {
            Part::One => self.solve_part_one(buf),
            Part::Two => self.solve_part_two(buf),
        };
        Ok(solution)
    }
}

pub mod y2015;
