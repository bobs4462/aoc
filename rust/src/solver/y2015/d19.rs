/// --- Day 19: Medicine for Rudolph ---
pub struct D19;

use std::collections::HashMap;

use crate::solver::{Solution, Solver};

impl Solver for D19 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let mut lines = input.split(|&c| c == b'\n');
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            if !(l.contains(&b'=') && l.contains(&b'>')) {
                return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
            }
        }
        let last = lines.next().unwrap();
        if !last.is_ascii() {
            return Err(format!(
                "Wrong line:\n{}",
                std::str::from_utf8(last).unwrap()
            ));
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        Solution::new("Number of lights on is: ", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        Solution::new("Number of lights on is: ", "".to_string())
    }
}

struct Reactor {
    transorms: HashMap<Vec<u8>, Vec<u8>>,
    molecule: Vec<u8>,
}

impl Reactor {
    fn new(input: Vec<u8>) -> Self {
        let mut transorms = HashMap::new();
        let mut lines = input.split(|&c| c == b'\n');
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut transorm = line.split(|&c| c == b' ');
            transorms.insert(
                transorm.next().unwrap().to_vec(),
                transorm.nth(1).unwrap().to_vec(),
            );
        }
        let molecule = lines.next().unwrap().to_vec();
        Reactor {
            transorms,
            molecule,
        }
    }

    fn calibrate(&self) -> usize {
        let mut sequence = self.molecule.iter().peekable();
        while let Some(v) = sequence.next() {
            let mut source = vec![v];
            if let Some(&n) = sequence.peek() {
                if n.is_ascii_lowercase() {}
                source.push(n);
                sequence.next();
            }
        }
        0
    }
}
