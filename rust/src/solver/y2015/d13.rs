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
        let mut optimum = 0;
        for p in permutator {
            optimum = optimum.max(Self::happines(p, &relations));
        }
        Solution::new("The optimum change in happines is:", optimum.to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let start = std::time::Instant::now();
        let (relations, mut attendees) = Self::relations(data.as_slice());
        attendees.push(b"Z");
        let permutator = Permutator::new(&mut attendees);
        let mut optimum = 0;
        for p in permutator {
            optimum = optimum.max(Self::happines(p, &relations));
        }
        println!("TIME: {:?}", start.elapsed());
        Solution::new("The optimum change in happines is:", optimum.to_string())
    }
}

impl D13 {
    fn relations(data: &[u8]) -> (HashMap<(&[u8], &[u8]), isize>, Vec<&[u8]>) {
        let mut relations = HashMap::new();
        let mut attendees = Vec::new();
        for line in data.split(|&c| c == b'\n') {
            let mut parts = line.split(|c| c.is_ascii_whitespace());
            // println!("LINE: {:?}", parts.clone().collect::<Vec<&[u8]>>());
            let attnd = parts.nth(0).unwrap();
            let modfr = match parts.nth(1) {
                Some(b"gain") => 1,
                Some(b"lose") => -1,
                wrong => panic!("Wrong modifier: {:?}", wrong),
            };
            let hppns: isize = {
                unsafe { std::str::from_utf8_unchecked(parts.nth(0).unwrap()) }
                    .parse()
                    .unwrap()
            };
            let nghbr = parts.nth(6).unwrap();
            relations.insert((attnd, &nghbr[..nghbr.len() - 1]), hppns * modfr);
            attendees.push(attnd);
        }
        attendees.dedup();
        (relations, attendees)
    }

    fn happines(table: &[&[u8]], relations: &HashMap<(&[u8], &[u8]), isize>) -> isize {
        let mut total = 0;
        for pair in table.windows(2) {
            // println!("KEY: {:?}");
            total += relations.get(&(pair[0], pair[1])).map_or_else(|| 0, |c| *c);
            total += relations.get(&(pair[1], pair[0])).map_or_else(|| 0, |c| *c);
        }
        total += relations
            .get(&(*(table.last().unwrap()), table[0]))
            .map_or_else(|| 0, |c| *c);
        total += relations
            .get(&(table[0], *(table.last().unwrap())))
            .map_or_else(|| 0, |c| *c);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    #[test]
    fn test_part_one() {
        let data = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#
            .to_string()
            .into();

        let solver = super::D13;
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "330")
    }
}
