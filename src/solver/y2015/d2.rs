/// --- Day 2: I Was Told There Would Be No Math ---
use crate::solver::{Solution, Solver};
use std::io::BufRead;

pub struct D2;

impl Solver for D2 {
    /// There's a need to validate that every line of the input consists of the following pattern
    /// DIGITSxDIGITSxDIGITS, and if at least one line violates this condition, then return error
    /// with the description of violating line
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        // helper function to validate each line
        fn pattern_match(entry: &[u8]) -> bool {
            // split the line at x char
            let parts = entry.split(|&c| c == b'x');
            let mut part_count = 0;
            for p in parts {
                part_count += 1;
                // make sure that each part only consists of ASCII digits
                for c in p {
                    if !c.is_ascii_digit() {
                        return false;
                    }
                }
            }
            // make sure, that there was three part for line
            if part_count != 3 {
                return false;
            }
            true
        }
        for (i, line) in input.lines().enumerate() {
            if let Ok(ln) = line {
                // check each line againts the pattern
                if !pattern_match(ln.as_bytes()) {
                    return Err(format!("INVALID LINE AT {}:\n{}", i, ln));
                }
            } else {
                return Err(format!("INVALID LINE AT {}:", i));
            }
        }
        Ok(())
    }
    /// Get dimensions, stick them into a fomula, sum up the results for every line
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut total: usize = 0;
        for line in data.lines() {
            let ln = line.expect("Coudn't read a line in data set");
            // Parse dimensions
            let [a, b, c] = self.dims(ln);
            total += (a * b + b * c + a * c) * 2 + (a * b).min(a * c).min(b * c)
        }
        Solution::new(
            "The total amount of wrapping paper is (square feet):",
            total.to_string(),
        )
    }
    /// The min perimeter can be obtained by taking two first min sides
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut total: usize = 0;
        for line in data.lines() {
            let ln = line.expect("Coudn't read a line in data set");
            let mut dims = self.dims(ln);
            // Perform ascending sort, two min elements will move to the front
            dims.sort();
            let [a, b, c] = dims;
            // a + b is the sum of two min elements
            total += (a + b) * 2 + a * b * c;
        }
        Solution::new("The total length of ribbon is (feet):", total.to_string())
    }
}

impl D2 {
    /// Helper function to parse and return 3 dimensions of the box
    fn dims(&self, ln: String) -> [usize; 3] {
        const OFST: u8 = 0x30;
        let mut dims = [0; 3];
        for (i, num) in ln.as_bytes().split(|&c| c == b'x').enumerate() {
            let mut n: usize = 0;
            let mut rank: isize = (num.len() - 1) as isize;
            const BASE: usize = 0xA;
            for &b in num {
                n += ((b - OFST) as usize) * BASE.pow(rank as u32);
                rank -= 1;
            }
            dims[i] = n;
        }
        dims
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        use crate::solver::Solver;
        const DATA: &'static str = r#"2x3x4
1x1x10
"#;
        let solver = super::D2 {};
        let res = solver.solve_part_one(DATA.as_bytes().to_vec());
        assert_eq!(res.value, "101");
    }
    #[test]
    fn test_part_two() {
        use crate::solver::Solver;
        const DATA: &'static str = r#"2x3x4
1x1x10
"#;
        let solver = super::D2 {};
        let res = solver.solve_part_two(DATA.as_bytes().to_vec());
        assert_eq!(res.value, "48");
    }
}
