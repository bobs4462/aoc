/// --- Day 16: Aunt Sue ---
pub struct D16;

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

impl Solver for D16 {
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

        Solution::new("The best cookie has a score", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new(
            "The best cookie with 500 calories has a score",
            "".to_string(),
        )
    }
}

struct AuntieSue {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl AuntieSue {
    fn new(line: &[u8], number: usize) -> Self {
        let parts = line
            .splitn(2, |&c| c == b':')
            .nth(1)
            .unwrap()
            .split(|&c| c == b',');
        let mut children: Option<usize> = None;
        let mut cats: Option<usize> = None;
        let mut samoyeds: Option<usize> = None;
        let mut pomeranians: Option<usize> = None;
        let mut akitas: Option<usize> = None;
        let mut vizslas: Option<usize> = None;
        let mut goldfish: Option<usize> = None;
        let mut trees: Option<usize> = None;
        let mut cars: Option<usize> = None;
        let mut perfumes: Option<usize> = None;
        for p in parts {
            let mut prop = p.split(|&c| c == b' ');

            match prop.next() {
                Some(b" children") => children = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" cats") => cats = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" samoyeds") => samoyeds = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" pomeranians") => {
                    pomeranians = Some((prop.next().unwrap()[1] - 48) as usize)
                }
                Some(b" akitas") => akitas = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" vizslas") => vizslas = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" goldfish") => goldfish = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" trees") => trees = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" cars") => cars = Some((prop.next().unwrap()[1] - 48) as usize),
                Some(b" perfumes") => perfumes = Some((prop.next().unwrap()[1] - 48) as usize),
                _ => panic!("Wrong input data!"),
            }
        }
        AuntieSue {
            number,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    }
}

impl D16 {
    fn aunties(input: Vec<u8>) -> Vec<AuntieSue> {
        let mut aunties = Vec::with_capacity(500);
        let lines = input.split(|&c| c == b'\n');
        for (i, l) in lines.enumerate() {
            aunties.push(AuntieSue::new(l, i));
        }
        aunties
    }
}
