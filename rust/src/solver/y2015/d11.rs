/// --- Day 11: Corporate Policy ---
/// There could be some distinct patterns in input, which can be easily solved analytically:
/// "l" - is some randome letter
/// 1. ZYXlllll, where Z, Y, X are distinct letters, which are not in incrementing sequence
/// 2. XYZlllll, where X, Y, Z are distinct letters, which are in incrementing sequence
/// 3. XXllllll AND lXXlllll, where XX is a pair of allowed letters
/// 4. XXYZllll AND XYZZllll
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

impl D11 {
    fn check(&self, pass: &[u8]) -> bool {
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

    fn try_solve(&self, pass: &[u8]) -> Option<Vec<u8>> {
        if self.check(pass) {
            return Some(pass.to_vec());
        }
        let mut next: Option<Vec<u8>> = None;
        let mut new: Vec<u8> = Vec::with_capacity(8);
        if (pass[2] - pass[1]) - (pass[1] - pass[0]) == 0 {
            new.push(pass[0]);
            new.push(pass[1]);
            new.push(pass[2]);
            if pass[4] > pass[5] {
                new.push(pass[4]);
                new.push(pass[4]);
            }
            if pass[6] > pass[7] {
                new.push(pass[6]);
                new.push(pass[6]);
            }
            next = Some(new);
        }
        next
    }
}
