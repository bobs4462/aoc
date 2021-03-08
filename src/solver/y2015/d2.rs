use crate::solver::Solver;
use std::io::BufRead;

pub(crate) struct D2;

impl Solver for D2 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        fn pattern_match(entry: &[u8]) -> bool {
            let parts = entry.split(|&c| c == b'x');
            let mut part_count = 0;
            for p in parts {
                part_count += 1;
                for c in p {
                    if !c.is_ascii_digit() {
                        return false;
                    }
                }
            }
            if part_count != 3 {
                return false;
            }
            true
        }
        for (i, line) in input.lines().enumerate() {
            if let Ok(ln) = line {
                if !pattern_match(ln.as_bytes()) {
                    return Err(format!("INVALID LINE AT {}:\n{}", i, ln));
                }
            } else {
                return Err(format!("INVALID LINE AT {}:", i));
            }
        }
        Ok(())
    }
    fn solve_part_one(&self, data: Vec<u8>) -> String {
        let mut total: usize = 0;
        for line in data.lines() {
            let ln = line.expect("Coudn't read a line in data set");
            let [a, b, c] = self.dims(ln);
            total += (a * b + b * c + a * c) * 2 + (a * b).min(a * c).min(b * c)
        }
        format!(
            "The total amount of wrapping paper is:\n{} square feet",
            total
        )
    }
    fn solve_part_two(&self, data: Vec<u8>) -> String {
        let mut total: usize = 0;
        for line in data.lines() {
            let ln = line.expect("Coudn't read a line in data set");
            let mut dims = self.dims(ln);
            dims.sort();
            let [a, b, c] = dims;
            total += (a + b) * 2 + a * b * c;
        }
        format!("The total length of ribbon is:\n{} feet", total)
    }
}

impl D2 {
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
        assert_eq!(
            res,
            String::from("The total amount of wrapping paper is:\n101 square feet",)
        );
    }
    #[test]
    fn test_part_two() {
        use crate::solver::Solver;
        const DATA: &'static str = r#"2x3x4
1x1x10
"#;
        let solver = super::D2 {};
        let res = solver.solve_part_two(DATA.as_bytes().to_vec());
        assert_eq!(res, String::from("The total length of ribbon is:\n48 feet"));
    }
}
