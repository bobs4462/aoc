/// --- Day 1: Not Quite Lisp ---
pub struct D1;

use crate::solver::{Solution, Solver};

impl Solver for D1 {
    /// The input consists only of left and right parentheses, so the validation is done to ensure
    /// that there're no other chars except for those two
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;
        for (i, &b) in input.iter().enumerate() {
            if b != LPAREN && b != RPAREN {
                return Err(format!(
                    "INVALID CHAR: '{}'\nAT POSISTION: {}",
                    char::from(b),
                    i
                ));
            }
        }
        Ok(())
    }
    /// Left parenthesis is an increment operation, and right one is decrement
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;

        let mut position = 0;
        for &b in data.iter() {
            match b {
                LPAREN => position += 1,
                RPAREN => position -= 1,
                _ => break,
            }
        }
        Solution::new("THE FINAL SANTA'S POSITION IS AT:", position.to_string())
    }
    /// There's only need to track the index of the current instruction, and once the absolute
    /// position becomes -1, just return the index
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;

        let mut position = 0;
        let mut below_ground = 0;
        for (i, &b) in data.iter().enumerate() {
            match b {
                LPAREN => position += 1,
                RPAREN => position -= 1,
                _ => break,
            }
            if position == -1 {
                below_ground = i + 1;
                break;
            }
        }
        Solution::new(
            "THE INSTRUCTION NUMBER TO GET SANTA BELOW GROUND IS:\n{}",
            below_ground.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        use crate::solver::Solver;
        let test_data = String::from("()()()))").into_bytes();
        let solver = super::D1 {};
        let res = solver.solve_part_one(test_data);
        assert_eq!(res.value, "-2");
    }
    #[test]
    fn test_part_two() {
        use crate::solver::Solver;
        let test_data = String::from("()()()))").into_bytes();
        let solver = super::D1 {};
        let res = solver.solve_part_two(test_data);
        assert_eq!(res.value, "7");
    }
}
