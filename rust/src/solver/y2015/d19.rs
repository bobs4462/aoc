/// --- Day 19: Medicine for Rudolph ---
pub struct D19;

use crate::solver::{Solution, Solver};

impl Solver for D19 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            if !l.iter().all(|&c| c == b'#' || c == b'.') {
                return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        Solution::new("Number of lights on is: ", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        Solution::new("Number of lights on is: ", "".to_string())
    }
}
