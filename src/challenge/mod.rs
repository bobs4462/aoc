use super::solver::Solver;
use std::fmt::{Display, Formatter};

pub struct Year {
    val: usize,
    pub(crate) days: Vec<Day>,
}

pub struct Day {
    val: u8,
    solver: Box<dyn Solver>,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year: {}", self.val)
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day: {}", self.val)
    }
}

impl Year {
    pub fn new(val: usize, days: Vec<Day>) -> Self {
        Year { val, days }
    }
    pub fn daysc(&self) -> isize {
        self.days.len() as isize
    }
    pub fn day(&self, index: usize) -> &Day {
        &self.days[index]
    }
}

impl Day {
    pub fn new(val: u8, solver: Box<dyn Solver>) -> Self {
        Day { val, solver }
    }

    pub fn solver(&self) -> &Box<dyn Solver> {
        &self.solver
    }
}
