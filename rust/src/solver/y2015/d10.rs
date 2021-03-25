/// --- Day 10: Elves Look, Elves Say ---
pub struct D10;

use crate::solver::{Solution, Solver};

impl Solver for D10 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for &c in input {
            if !c.is_ascii_digit() {
                return Err(format!(
                    "The character is not an ASCII digit: {}",
                    std::char::from_u32(c as u32).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        // const RPS: u8 = 5;
        const RPS: u8 = 40;
        let mut val: Vec<u8> = data;
        for _ in 0..RPS {
            val = reevaluate(val);
        }
        Solution::new("Number of repetitions:", val.len().to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        const RPS: u8 = 50;
        let mut val: Vec<u8> = data;
        for _ in 0..RPS {
            val = reevaluate(val);
        }
        Solution::new("Number of repetitions:", val.len().to_string())
    }
}

fn reevaluate(input: Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();

    let mut last: u8 = input[0];
    let mut counter = 1;
    for &c in &input[1..] {
        if last == c {
            counter += 1;
        } else {
            res.push(counter + 0x30);
            res.push(last);
            counter = 1;
        }
        last = c;
    }
    res.extend(counter.to_string().as_bytes().iter());
    res.push(last);
    res
}

#[cfg(test)]
mod tests {
    use super::{Solver, D10};
    #[test]
    fn test_reevaluate() {
        let data = b"1111121".to_vec();
        let res = super::reevaluate(data);
        println!("STRING: {}", std::str::from_utf8(res.as_slice()).unwrap());
        assert_eq!(res, b"511211".to_vec());
    }

    #[test]
    fn test_part_one() {
        let solver = D10;
        let data = b"1".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "6");
    }
}
