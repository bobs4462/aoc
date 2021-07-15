/// --- Day 19: Medicine for Rudolph ---
pub struct D19;

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
        let reactor = Reactor::new(&data);
        let unique = reactor.calibrate();

        Solution::new("Unique number of replacements", unique.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut reactor = Reactor::new(&data);
        let steps = reactor.synthesize();

        Solution::new(
            "Number of steps to produce the molecule: ",
            steps.to_string(),
        )
    }
}

struct Reactor {
    original: Vec<u8>,
    transforms: Vec<(Vec<u8>, Vec<u8>)>,
    electrons: Vec<Vec<u8>>,
}

impl Reactor {
    fn new(setup: &[u8]) -> Self {
        let lines = setup.split(|&b| b == b'\n');

        let mut transforms = Vec::with_capacity(64);

        let original = lines.rev().next().unwrap().to_owned();

        let lines = setup.split(|&b| b == b'\n');

        let mut electrons = Vec::with_capacity(8);

        for l in lines {
            if l.is_empty() {
                break;
            }

            let mut l = l.split(|c| c.is_ascii_whitespace());

            let source = l.next().unwrap().to_vec();
            let dest = l.last().unwrap().to_vec();

            if source.len() == 1 && source[0] == b'e' {
                electrons.push(Vec::clone(&dest));
            }

            transforms.push((source, dest));
        }

        Reactor {
            original,
            transforms,
            electrons,
        }
    }

    fn synthesize(&mut self) -> usize {
        let mut staged = Vec::clone(&self.electrons);
        let mut intermediaries = Vec::new();
        let mut step = 2;

        'outer: loop {
            println!(
                "STEP: {}: {:?}",
                step,
                staged
                    .iter()
                    .map(|s| std::str::from_utf8(s).unwrap())
                    .collect::<Vec<&str>>()
            );
            for s in &staged {
                let mut molecules = self.construct(&s);
                if molecules.iter().any(|m| *m == self.original) {
                    break 'outer;
                }
                intermediaries.append(&mut molecules);
            }
            intermediaries.sort();
            intermediaries.dedup();

            staged = intermediaries;
            intermediaries = Vec::new();

            step += 1;
        }
        step
    }

    fn calibrate(&self) -> usize {
        let variations = self.construct(&self.original);

        variations.len()
    }

    fn construct(&self, molecule: &Vec<u8>) -> Vec<Vec<u8>> {
        let mut molecules = Vec::new();
        for (s, d) in &self.transforms {
            let mut i = 0;

            for (j, b) in molecule.iter().enumerate() {
                if b != unsafe { s.get_unchecked(i) } {
                    i = 0;
                    if b == unsafe { s.get_unchecked(i) } {
                        i = 1;
                    }
                    continue;
                }
                if i == s.len() - 1 {
                    let chain = molecule[0..j + 1 - s.len()]
                        .chain(&d[..])
                        .chain(&molecule[j + 1..])
                        .bytes()
                        .flatten();
                    molecules.push(chain.collect::<Vec<u8>>());

                    i = 0;
                } else {
                    i += 1;
                }
            }
        }
        molecules.sort();
        molecules.dedup();
        molecules
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

    #[test]
    fn test_part_two() {
        let solver = super::D19;
        let data = r#"e => H
e => O
H => HO
H => OH
O => HH

HOH"#
            .as_bytes()
            .to_vec();

        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "3");

        let data = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "6");
    }
}
