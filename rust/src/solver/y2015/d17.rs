/// --- Day 17: No Such Thing as Too Much ---
pub struct D17;

use crate::solver::{Solution, Solver};

impl Solver for D17 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            if !l.iter().all(|&c| c.is_ascii_digit()) {
                return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let containers = D17::containers(data);
        let subsets = 2u32.pow(containers.len() as u32);
        let mut fit = 0;
        for st in 0..subsets {
            let mut volume = 0;
            for (i, v) in containers.iter().enumerate() {
                volume += v * D17::on(st, i) as usize;
            }
            fit += 1 * (volume == 150) as usize;
        }
        Solution::new("Number of perfect fits is: ", fit.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let containers = D17::containers(data);
        let subsets = 2u32.pow(containers.len() as u32);
        let mut quantity = Vec::with_capacity(1024);
        let mut min = usize::MAX;
        for st in 0..subsets {
            let mut volume = 0;
            let mut number = 0;
            for (i, v) in containers.iter().enumerate() {
                let on = D17::on(st, i) as usize;
                volume += v * on;
                number += 1 * on;
            }
            if volume == 150 {
                quantity.push(number);
                min = min.min(number);
            }
        }
        let mins = quantity.iter().filter(|&&q| q == min).count();
        Solution::new("Min number of perfect fits is: ", mins.to_string())
    }
}

impl D17 {
    fn containers(input: Vec<u8>) -> Vec<usize> {
        let mut containers = Vec::with_capacity(32);

        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            containers.push(std::str::from_utf8(l).unwrap().parse::<usize>().unwrap());
        }

        containers
    }

    #[inline]
    fn on(subset: u32, ordinal: usize) -> bool {
        let mask = 2u32.pow(ordinal as u32);
        (subset & mask) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::{Solver, D17};

    #[test]
    fn test_part_one() {
        // Change the check volume from 150 to 25 to test
        let data = b"20\n10\n15\n5\n5".to_vec();
        let solver = D17;
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "4");
    }

    #[test]
    fn test_part_two() {
        // Change the check volume from 150 to 25 to test
        let data = b"20\n10\n15\n5\n5".to_vec();
        let solver = D17;
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "3");
    }

    #[test]
    fn test_on() {
        assert!(D17::on(1, 0));
        assert!(D17::on(2, 1));
        assert!(D17::on(3, 0));
        assert!(D17::on(3, 1));
    }
}
