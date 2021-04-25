/// --- Day 15: Science for Hungry People ---
pub struct D15;

use crate::solver::{Solution, Solver};

impl Solver for D15 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            let components: Vec<&[u8]> = l.split(|&c| c == b':').collect();
            if components.len() != 2 || components[1].split(|&c| c == b',').count() != 5 {
                return Err(format!(
                    "Invalid line:\n{}",
                    std::str::from_utf8(l).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        Solution::new("", format!(""))
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        Solution::new("", format!(""))
    }
}
