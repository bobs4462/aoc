/// --- Day 13: Knights of the Dinner Table ---
pub struct D13;

use std::collections::HashMap;

use crate::solver::{utils::Permutator, Solution, Solver};

impl Solver for D13 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for (i, line) in input.split(|&c| c == b'\n').enumerate() {
            let l: Vec<&[u8]> = line.split(|c| c.is_ascii_whitespace()).collect();
            if l.len() != 11 {
                return Err(format!(
                    "Wrong line at {}: {}",
                    i,
                    std::str::from_utf8(line).unwrap()
                ));
            }

            if let Err(_) = std::str::from_utf8(l[3]).unwrap().parse::<usize>() {
                return Err(format!(
                    "Expected a numbe at 4th word, line at {}: {}",
                    i,
                    std::str::from_utf8(l[3]).unwrap()
                ));
            }
        }
        Ok(())
    }
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let (relations, mut attendees) = Self::relations(data.as_slice());
        let permutator = Permutator::new(&mut attendees);
        for p in permutator {}
        Solution::new("", "".to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }
}

impl D13 {
    fn relations(data: &[u8]) -> (HashMap<(&[u8], &[u8]), isize>, Vec<&[u8]>) {
        let mut relations = HashMap::new();
        let mut attendees = Vec::new();
        for line in data.split(|&c| c == b'\n') {
            let mut parts = line.split(|c| c.is_ascii_whitespace());
            let attnd = parts.nth(0).unwrap();
            let modfr = match parts.nth(2) {
                Some(b"gain") => 1,
                Some(b"lose") => -1,
                wrong => panic!("Wrong modifier: {:?}", wrong),
            };
            let hppns: isize = {
                unsafe { std::str::from_utf8_unchecked(parts.nth(4).unwrap()) }
                    .parse()
                    .unwrap()
            };
            let nghbr = parts.nth(10).unwrap();
            relations.insert((attnd, nghbr), hppns * modfr);
            attendees.push(attnd);
        }
        attendees.dedup();
        (relations, attendees)
    }

    fn happines(table: &[&[u8]], relations: &HashMap<(&[u8], &[u8]), isize>) -> isize {
        let mut total = 0;
        for pair in table.windows(2) {
            total += relations.get(&(pair[0], pair[0])).expect("Key not found");
        }
        total += relations
            .get(&(table[0], *(table.last()).unwrap()))
            .expect("Key not found");
        total
    }
}
