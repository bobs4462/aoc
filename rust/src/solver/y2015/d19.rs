/// --- Day 19: Medicine for Rudolph ---
pub struct D19;

use std::collections::HashSet;
use std::io::Read;

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
        let mut reactor = Reactor::new(&data);
        let unique = reactor.calibrate();

        Solution::new("Unique number of replacements", unique.to_string())
    }

    fn solve_part_two(&self, _data: Vec<u8>) -> Solution {
        Solution::new("Number of lights on is: ", "".to_string())
    }
}

fn shash<T: Iterator<Item = u8>>(string: T) -> usize {
    let mut res = 0;

    for (i, b) in string.enumerate() {
        res += ((b as usize ^ i) + (b as usize & i)) * i;
    }

    res
}

struct Reactor {
    original: Vec<u8>,
    transforms: Vec<(Vec<u8>, Vec<u8>)>,
    hashes: HashSet<Vec<u8>>,
}

impl Reactor {
    fn new(setup: &[u8]) -> Self {
        let lines = setup.split(|&b| b == b'\n');

        let mut transforms = Vec::with_capacity(64);

        let original = lines.rev().next().unwrap().to_owned();

        let lines = setup.split(|&b| b == b'\n');

        for l in lines {
            if l.is_empty() {
                break;
            }
            let mut source = Vec::new();
            let mut dest = Vec::new();

            let mut l = l.iter();

            while let Some(b) = l.next() {
                if b.is_ascii_whitespace() {
                    break;
                }
                source.push(*b);
            }
            while let Some(b) = l.next() {
                if b.is_ascii_whitespace() {
                    break;
                }
            }
            while let Some(b) = l.next() {
                dest.push(*b);
            }
            transforms.push((source, dest));
        }

        Reactor {
            original,
            transforms,
            hashes: HashSet::new(),
        }
    }
    fn calibrate(&mut self) -> usize {
        for (s, d) in &self.transforms {
            let mut i = 0;

            for (j, b) in self.original.iter().enumerate() {
                if b != unsafe { s.get_unchecked(i) } {
                    i = 0;
                    if b == unsafe { s.get_unchecked(i) } {
                        i = 1;
                    }
                    continue;
                }
                if i == s.len() - 1 {
                    let chain = self.original[0..j + 1 - s.len()]
                        .chain(&d[..])
                        .chain(&self.original[j + 1..])
                        .bytes()
                        .flatten();
                    self.hashes.insert(chain.collect());
                    i = 0;
                } else {
                    i += 1;
                }
            }
        }
        self.hashes.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reactor_new() {
        use super::Reactor;
        let r = Reactor::new(
            r#"H => HO
H => OH
O => HH

HOH"#
                .to_owned()
                .as_bytes(),
        );
        assert_eq!(r.original, b"HOH");
        assert_eq!(r.transforms[0].0, vec![b'H']);
        assert_eq!(r.transforms[0].1, vec![b'H', b'O']);
        assert_eq!(r.transforms[1].0, vec![b'H']);
        assert_eq!(r.transforms[1].1, vec![b'O', b'H']);
        assert_eq!(r.transforms[2].0, vec![b'O']);
        assert_eq!(r.transforms[2].1, vec![b'H', b'H']);
    }

    use crate::solver::Solver;

    #[test]
    fn test_part_one() {
        let solver = super::D19;
        let mut data = r#"H => HO
H => OH
O => HH

HOH"#
            .as_bytes()
            .to_vec();

        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "4");
        data = r#"H => HO
H => OH
O => HH

HOHOHO"#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "7");
    }
}
