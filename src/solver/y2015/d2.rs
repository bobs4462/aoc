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
        const OFST: u8 = 0x30;
        let mut total: usize = 0;
        for line in data.lines() {
            let mut dims = [0; 3];
            let ln = line.expect("Coudn't read a line in data set");
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
            let bt = dims[0] * dims[1] * 2;
            let lr = dims[1] * dims[2] * 2;
            let fb = dims[0] * dims[2] * 2;
            dims[0] = bt / 2;
            dims[1] = lr / 2;
            dims[2] = fb / 2;
            let box_paper = bt + lr + fb;
            let extra_paper = dims.iter().min().expect("Couldn't find minimum value");
            total += box_paper + *extra_paper;
        }
        format!(
            "The total amount of wrapping paper is:\n{} square feet",
            total
        )
    }
    fn solve_part_two(&self, data: Vec<u8>) -> String {
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
        format!(
            "THE INSTRUCTION NUMBER TO GET SANTA BELOW GROUND IS:\n{}",
            below_ground
        )
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
        let test_data = String::from("()()()))").into_bytes();
        let solver = super::D2 {};
        let res = solver.solve_part_two(test_data);
        assert_eq!(
            res,
            String::from("THE INSTRUCTION NUMBER TO GET SANTA BELOW GROUND IS:\n7")
        );
    }
}
