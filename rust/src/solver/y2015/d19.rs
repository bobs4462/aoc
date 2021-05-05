/// --- Day 19: Medicine for Rudolph ---
pub struct D19;

use std::collections::{HashMap, HashSet};

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
    transorms: Vec<(Vec<u8>, Vec<u8>)>,
    molecule: Vec<u8>,
    transmutations: HashSet<String>,
}

impl Reactor {
    fn new(input: Vec<u8>) -> Self {
        let mut transorms = Vec::with_capacity(64);
        let mut lines = input.split(|&c| c == b'\n');
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut transorm = line.split(|&c| c == b' ');
            transorms.push((
                transorm.next().unwrap().to_vec(),
                transorm.nth(1).unwrap().to_vec(),
            ));
        }
        let molecule = lines.next().unwrap().to_vec();
        Reactor {
            transorms,
            molecule,
            transmutations: HashSet::with_capacity(1024),
        }
    }

    fn calibrate(&self) -> usize {
        for t in &self.transorms {
            let mut sequence = self.molecule.iter().enumerate();

            while let Some((i, l)) = sequence.next() {}
        }
        0
    }
}
