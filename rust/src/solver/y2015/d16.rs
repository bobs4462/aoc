/// --- Day 16: Aunt Sue ---
pub struct D16;

use crate::solver::{Solution, Solver};

// const CHILDREN: usize = 3;
// const CATS: usize = 7;
// const SAMOYEDS: usize = 2;
// const POMERANIANS: usize = 3;
// const AKITAS: usize = 0;
// const VIZSLAS: usize = 0;
// const GOLDFISH: usize = 5;
// const TREES: usize = 3;
// const CARS: usize = 2;
// const PERFUMES: usize = 1;

impl Solver for D16 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            let mut parts = l.split(|&c| c == b':');
            parts.next();
            parts.next();
            for p in parts.step_by(2) {
                if !p[1].is_ascii_digit() {
                    return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
                }
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("The best cookie has a score", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new(
            "The best cookie with 500 calories has a score",
            "".to_string(),
        )
    }
}
