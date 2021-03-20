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

impl From<isize> for Part {
    fn from(val: isize) -> Self {
        match val {
            0 => Part::One,
            _ => Part::Two,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum SolveErrorType {
    ReadError,
    ValidationError,
}
#[derive(Debug, PartialOrd, PartialEq)]
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

pub struct Solution {
    message: &'static str,
    value: String,
}

impl Solution {
    pub fn new(message: &'static str, value: String) -> Self {
        Solution { message, value }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.message, self.value)
    }
}

pub trait Solver: Sync {
    fn validate(&self, input: &[u8]) -> Result<(), String>;
    fn solve_part_one(&self, data: Vec<u8>) -> Solution;
    fn solve_part_two(&self, data: Vec<u8>) -> Solution;
    fn solve(&self, mut f: File, part: Part) -> Result<String, SolveError> {
        let mut buf = Vec::with_capacity(1000);
        const NWLN: u8 = 0xA;
        match f.read_to_end(&mut buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(SolveError {
                    etype: SolveErrorType::ReadError,
                    message: e.to_string(),
                })
            }
        };
        loop {
            if let Some(&b) = buf.last() {
                if b == NWLN {
                    buf.pop();
                } else {
                    break;
                }
            }
        }
        if let Err(dscrpt) = self.validate(buf.as_slice()) {
            return Err(SolveError {
                etype: SolveErrorType::ValidationError,
                message: dscrpt,
            });
        }
        let solution = match part {
            Part::One => self.solve_part_one(buf),
            Part::Two => self.solve_part_two(buf),
        };
        Ok(solution.to_string())
    }
}

pub mod y2015;
