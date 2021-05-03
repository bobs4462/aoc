/// --- Day 17: No Such Thing as Too Much ---
pub struct D17;

use crate::solver::{Solution, Solver};

const CHILDREN: usize = 3;
const CATS: usize = 7;
const SAMOYEDS: usize = 2;
const POMERANIANS: usize = 3;
const AKITAS: usize = 0;
const VIZSLAS: usize = 0;
const GOLDFISH: usize = 5;
const TREES: usize = 3;
const CARS: usize = 2;
const PERFUMES: usize = 1;

impl Solver for D17 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            let mut parts = l.splitn(2, |&c| c == b':');
            parts.next();
            let parts = parts.next().unwrap().split(|&c| c == b',');
            for p in parts {
                if !p.split(|&c| c == b':').nth(1).unwrap()[1].is_ascii_digit() {
                    return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
                }
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let aunties = D16::aunties(data);
        let mut the_aunt: &AuntieSue = &aunties[0];
        let mut best_match = 0;
        for a in &aunties {
            let mut matches = 0;
            if let Some(v) = a.children {
                if v == CHILDREN {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.cats {
                if v == CATS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.samoyeds {
                if v == SAMOYEDS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.pomeranians {
                if v == POMERANIANS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.akitas {
                if v == AKITAS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.vizslas {
                if v == VIZSLAS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.goldfish {
                if v == GOLDFISH {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.cars {
                if v == CARS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.trees {
                if v == TREES {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.perfumes {
                if v == PERFUMES {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if best_match < matches {
                the_aunt = a;
                best_match = matches;
            }
        }
        Solution::new("The aunt Sue has a number: ", the_aunt.number.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let aunties = D16::aunties(data);
        let mut the_aunt: &AuntieSue = &aunties[0];
        let mut best_match = 0;
        for a in &aunties {
            let mut matches = 0;
            if let Some(v) = a.children {
                if v == CHILDREN {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.cats {
                if v > CATS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.samoyeds {
                if v == SAMOYEDS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.pomeranians {
                if v < POMERANIANS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.akitas {
                if v == AKITAS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.vizslas {
                if v == VIZSLAS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.goldfish {
                if v < GOLDFISH {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.cars {
                if v == CARS {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.trees {
                if v > TREES {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if let Some(v) = a.perfumes {
                if v == PERFUMES {
                    matches += 1;
                } else {
                    continue;
                }
            }
            if best_match < matches {
                the_aunt = a;
                best_match = matches;
            }
        }
        Solution::new("The aunt Sue has a number: ", the_aunt.number.to_string())
    }
}
