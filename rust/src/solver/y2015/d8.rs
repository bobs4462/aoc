/// --- Day 8: Matchsticks ---
pub struct D8;

use crate::solver::{Solution, Solver};

impl Solver for D8 {
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|&c| c == b'\n');
        let mut total: u32 = 0;
        let mut inmemory: u32 = 0;
        for l in lines {
            let mut liter = l.iter();
            loop {
                if let Some(&c) = liter.next() {
                    if c == b'\\' {
                        match liter.next().unwrap() {
                            b'\\' | b'\"' => total += 1,
                            b'x' => {
                                liter.next();
                                liter.next();
                                total += 3;
                            }
                            _ => panic!("Wrong escape sequence!"),
                        }
                    }
                    total += 1;
                    inmemory += 1;
                } else {
                    break;
                }
            }
            inmemory -= 2;
            // println!("TOTAL: {}", total);
            // println!("INMEM: {}", inmemory);
        }
        Solution::new(
            "Total char count minus in memory char count is:",
            (total - inmemory).to_string(),
        )
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|&c| c == b'\n');
        let mut orignal: u32 = 0;
        let mut escaped: u32 = 0;
        for l in lines {
            let mut liter = l.iter();
            loop {
                if let Some(&c) = liter.next() {
                    if c == b'\\' || c == b'\"' {
                        escaped += 1;
                    }
                    orignal += 1;
                    escaped += 1;
                } else {
                    break;
                }
            }
            escaped += 2;
            // println!("ESCP: {}", escaped);
            // println!("ORIG: {}", orignal);
        }
        Solution::new(
            "Ecaped char count minus orignal char count is:",
            (escaped - orignal).to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let solver = super::D8;
        let data = b"\"aaa\\\"aaa\"".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "3");
        let data = r#"""
"abc"
"aaa\"aaa"
"\x27""#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "12");
    }

    #[test]
    fn test_part_two() {
        let solver = super::D8;
        let data = b"\"aaa\\\"aaa\"".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "6");
        let data = r#"""
"abc"
"aaa\"aaa"
"\x27""#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "19");
    }
}
