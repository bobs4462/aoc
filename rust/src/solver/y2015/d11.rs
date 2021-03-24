/// --- Day 11: Corporate Policy ---
pub struct D11;

use crate::solver::{Solution, Solver};

impl Solver for D11 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for &c in input {
            if !c.is_ascii_alphabetic() {
                return Err(format!(
                    "The character is not an ASCII digit: {}",
                    std::char::from_u32(c as u32).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }
}

fn check(pass: &[u8]) -> bool {
    let mut cond1 = false;
    for straight in pass.windows(3) {
        cond1 = (straight[2] - straight[1]) - (straight[1] - straight[0]) == 0;
    }
    if !cond1 {
        return false;
    }
    let mut iter = pass.windows(2);
    let mut pairs: usize = 0;
    while let Some(v) = iter.next() {
        if v[0] == v[1] {
            iter.next();
            pairs += 1;
        }
    }
    pairs > 1
}
