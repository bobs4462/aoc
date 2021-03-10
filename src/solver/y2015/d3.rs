use crate::solver::{Solution, Solver};
use std::collections::HashSet;

pub(crate) struct D3;

struct Coords {
    current: (isize, isize),
    entries: HashSet<(isize, isize)>,
}

impl Default for Coords {
    fn default() -> Self {
        let current = (0, 0);
        let entries = {
            let mut hs = HashSet::new();
            hs.insert(current);
            hs
        };
        Coords { current, entries }
    }
}

impl Coords {
    fn vm(&mut self, modifier: isize) {
        self.current.1 += modifier;
        self.entries.insert(self.current);
    }
    fn hm(&mut self, modifier: isize) {
        self.current.0 += modifier;
        self.entries.insert(self.current);
    }

    fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Solver for D3 {
    /// Only need to validate that every byte is just one of the: ^ v < >
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let mut valid = HashSet::new();
        valid.insert(b'^');
        valid.insert(b'v');
        valid.insert(b'<');
        valid.insert(b'>');
        for (i, b) in input.iter().enumerate() {
            if !valid.contains(b) {
                return Err(format!("Invalid value at position {}: {}", i, b));
            }
        }
        Ok(())
    }
    /// Iterate over the move instructions, every single move, record the current position, hashset
    /// guarantees that every position will be unique
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut coords = Coords::default();
        for &b in data.iter() {
            match b {
                b'^' => coords.vm(1),
                b'v' => coords.vm(-1),
                b'<' => coords.hm(-1),
                b'>' => coords.hm(1),
                _ => {}
            }
        }
        Solution::new(
            "The total of houses, santa visited is:",
            coords.count().to_string(),
        )
    }
    /// The min perimeter can be obtained by taking two first min sides
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        Solution::new(
            "The total amount of wrapping paper is (square feet):",
            "".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        use crate::solver::Solver;
        let solver = super::D3 {};
        let mut data: &'static str = "<<<<^^^^";
        let res = solver.solve_part_one(data.as_bytes().to_vec());
        assert_eq!(res.value, "9");
        data = ">";
        let res = solver.solve_part_one(data.as_bytes().to_vec());
        assert_eq!(res.value, "2");
        data = "^>v<";
        let res = solver.solve_part_one(data.as_bytes().to_vec());
        assert_eq!(res.value, "4");
        data = "^v^v^v^v^v";
        let res = solver.solve_part_one(data.as_bytes().to_vec());
        assert_eq!(res.value, "2");
    }
    // #[test]
    // fn test_part_two() {
    // use crate::solver::Solver;
    // const DATA: &'static str = r#"2x3x4
    // 1x1x10
    // "#;
    // let solver = super::D2 {};
    // let res = solver.solve_part_two(DATA.as_bytes().to_vec());
    // assert_eq!(res.value, "48");
    // }
}
