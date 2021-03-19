/// --- Day 5: Doesn't He Have Intern-Elves For This? ---
pub struct D6;

struct Instruction {
    action: Action,
    range: Range,
}

struct Range {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy)]
enum Action {
    Off = 0,
    On = 1,
    Toggle,
}

impl Instruction {
    fn new(instruction: &[u8]) -> Self {
        let tokens: Vec<&[u8]> = instruction.split(|c| c.is_ascii_whitespace()).collect();
        let instruction = if tokens.len() == 5 {
            let range = unsafe {
                Range::new(
                    std::str::from_utf8_unchecked(tokens[2]),
                    std::str::from_utf8_unchecked(tokens[4]),
                )
            };
            match tokens[1] {
                b"on" => Instruction {
                    action: Action::On,
                    range,
                },
                b"off" => Instruction {
                    action: Action::Off,
                    range,
                },
                _ => panic!("Wrong instruction!"),
            }
        } else {
            Instruction {
                action: Action::Toggle,
                range: unsafe {
                    Range::new(
                        std::str::from_utf8_unchecked(tokens[1]),
                        std::str::from_utf8_unchecked(tokens[3]),
                    )
                },
            }
        };
        instruction
    }
}

impl Range {
    fn new(start: &str, end: &str) -> Self {
        let start: Vec<&str> = start.split(',').collect();
        let start: (usize, usize) = (start[0].parse().unwrap(), start[1].parse().unwrap());
        let end: Vec<&str> = end.split(',').collect();
        let end: (usize, usize) = (end[0].parse().unwrap(), end[1].parse().unwrap());
        Range { start, end }
    }
    #[inline]
    fn inrange(&self, coords: (usize, usize)) -> bool {
        self.start.0 <= coords.0
            && self.end.0 >= coords.0
            && self.start.1 <= coords.1
            && self.end.1 >= coords.1
    }

    fn split(num: usize, chunks: usize) -> Vec<(usize, usize)> {
        let chunk_size = num / chunks;
        let mut result = Vec::with_capacity(chunks);
        let mut start;
        let mut end;
        for i in 0..chunks {
            start = i * chunk_size;
            end = chunk_size * (i + 1);
            if num - end < chunk_size {
                end += num - end;
            }
            result.push((start, end));
        }
        result
    }
}
use crate::solver::{Solution, Solver};
use num_cpus;
use std::sync::Arc;
use std::{sync::mpsc, thread};

impl Solver for D6 {
    /// Just validate that the input is alphanumeric in ASCII
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');

        for (i, l) in lines.enumerate() {
            match l.split(|c| c.is_ascii_whitespace()).count() {
                4 | 5 => continue,
                _ => {
                    return Err(format!(
                        "Line at {} is invalid: {}",
                        i,
                        std::str::from_utf8(l).unwrap()
                    ))
                }
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut instructions = Vec::with_capacity(300);
        for l in data.split(|&c| c == b'\n').rev() {
            instructions.push(Instruction::new(l));
        }
        let instructions = Arc::new(instructions);
        let (tx, rx) = mpsc::channel::<i32>();
        let cores = num_cpus::get() - 1;
        let mut ranges = Range::split(1000, cores);
        for _ in 0..cores {
            let instructions = Arc::clone(&instructions);
            let tx = tx.clone();
            let r = ranges.pop().expect("Not enough ranges!");
            thread::spawn(move || {
                let mut lit = 0;
                for x in r.0..r.1 {
                    'y: for y in 0..1000 {
                        let mut toggle = 0;
                        for instr in instructions.iter() {
                            if !instr.range.inrange((x, y)) {
                                continue;
                            }
                            match instr.action {
                                Action::On | Action::Off => {
                                    lit += (toggle + instr.action as i32) % 2;
                                    continue 'y;
                                }
                                Action::Toggle => toggle += 1,
                            }
                        }
                        lit += toggle % 2;
                    }
                }
                tx.send(lit).expect("Couldn't send partial result!");
            });
        }

        let mut lit = 0;
        drop(tx);
        while let Ok(value) = rx.recv() {
            lit += value;
        }

        Solution::new("The number of lights lit:", lit.to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut instructions = Vec::with_capacity(300);
        for l in data.split(|&c| c == b'\n') {
            instructions.push(Instruction::new(l));
        }
        let instructions = Arc::new(instructions);
        let (tx, rx) = mpsc::channel::<u32>();
        let cores = num_cpus::get() - 1;
        let mut ranges = Range::split(1000, cores);
        for _ in 0..cores {
            let instructions = Arc::clone(&instructions);
            let tx = tx.clone();
            let r = ranges.pop().expect("Not enough ranges!");
            thread::spawn(move || {
                let mut lit: u32 = 0;
                for x in r.0..r.1 {
                    for y in 0..1000 {
                        let mut brigtness = 0;
                        for instr in instructions.iter() {
                            if !instr.range.inrange((x, y)) {
                                continue;
                            }
                            match instr.action {
                                Action::On => brigtness += 1,
                                Action::Off => brigtness -= if brigtness > 0 { 1 } else { 0 },
                                Action::Toggle => brigtness += 2,
                            }
                        }
                        lit += brigtness;
                    }
                }
                tx.send(lit).expect("Couldn't send partial result!");
            });
        }

        let mut lit = 0;
        drop(tx);
        while let Ok(value) = rx.recv() {
            lit += value;
        }

        Solution::new("The number of lights lit:", lit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let solver = super::D6;
        let mut data = b"turn on 0,0 through 999,999".to_vec();
        let mut res = solver.solve_part_one(data);
        assert_eq!(res.value, "1000000");
        data = b"toggle 0,0 through 999,0".to_vec();
        res = solver.solve_part_one(data);
        assert_eq!(res.value, "1000");
        data = b"turn off 499,499 through 500,500".to_vec();
        res = solver.solve_part_one(data);
        assert_eq!(res.value, "0");
    }
    #[test]
    fn test_split() {
        assert_eq!(
            super::Range::split(1200, 3),
            vec![(0, 400), (400, 800), (800, 1200)]
        );
        assert_eq!(
            super::Range::split(4000, 3),
            vec![(0, 1333), (1333, 2666), (2666, 4000)]
        );
    }
    #[test]
    fn test_part_two() {
        let solver = super::D6;
        let mut data = b"turn on 0,0 through 0,0".to_vec();
        let mut res = solver.solve_part_two(data);
        assert_eq!(res.value, "1");
        data = b"toggle 0,0 through 999,999".to_vec();
        res = solver.solve_part_two(data);
        assert_eq!(res.value, "2000000");
    }
}
