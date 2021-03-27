/// --- Day 13: Knights of the Dinner Table ---
pub struct D13;

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
        let mut attendees = Self::relations(data.as_slice());
        let permutator = Permutator::new(&mut attendees);
        let mut optimum = 0;
        for p in permutator {
            optimum = optimum.max(Self::happines(p));
        }
        Solution::new("The optimum change in happines is:", optimum.to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut attendees = Self::relations(data.as_slice());
        attendees.insert(
            0,
            Attendee {
                attitudes: Vec::new(),
                id: usize::MAX,
            },
        );
        let permutator = Permutator::new(&mut attendees);
        let mut optimum = 0;
        for p in permutator {
            optimum = optimum.max(Self::happines(p));
        }
        Solution::new("The optimum change in happines is:", optimum.to_string())
    }
}

#[derive(Debug)]
struct Attendee {
    attitudes: Vec<isize>,
    id: usize,
}

impl PartialEq for Attendee {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl Eq for Attendee {}

impl PartialOrd for Attendee {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Attendee {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl D13 {
    fn relations(data: &[u8]) -> Vec<Attendee> {
        let mut attendees: Vec<Attendee> = Vec::with_capacity(10);
        let mut prev: &[u8] = &[];
        let mut id: usize = 0;
        for line in data.split(|&c| c == b'\n') {
            let mut parts = line.split(|c| c.is_ascii_whitespace());
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
            if attnd != prev {
                if let Some(a) = attendees.last_mut() {
                    a.attitudes.insert(id - 1, 0);
                }
                attendees.push(Attendee {
                    attitudes: vec![hppns * modfr],
                    id,
                });
                id += 1;
            } else {
                attendees.last_mut().unwrap().attitudes.push(hppns * modfr);
            }
            prev = attnd;
        }
        attendees.last_mut().unwrap().attitudes.insert(id - 1, 0);
        attendees
    }

    fn happines(table: &[Attendee]) -> isize {
        let mut total = 0;
        let len = table.len();
        for i in 0..len {
            unsafe {
                let current = table.get_unchecked(i).id;
                let next = table.get_unchecked((i + 1) % len).id;
                if current == usize::MAX || next == usize::MAX {
                    continue;
                }
                total += table.get_unchecked(i).attitudes.get_unchecked(next);
                total += table
                    .get_unchecked((i + 1) % len)
                    .attitudes
                    .get_unchecked(current);
            }
        }
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
