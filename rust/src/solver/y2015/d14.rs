/// --- Day 14: Reindeer Olympics ---
pub struct D14;

use crate::solver::{Solution, Solver};

const TIME: usize = 2503;

impl Solver for D14 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for (i, line) in input.split(|&c| c == b'\n').enumerate() {
            let l: Vec<&[u8]> = line.split(|c| c.is_ascii_whitespace()).collect();
            if l.len() != 15 {
                return Err(format!(
                    "Wrong line at {}: {}",
                    i,
                    std::str::from_utf8(line).unwrap()
                ));
            }

            if let Err(_) = std::str::from_utf8(l[3]).unwrap().parse::<usize>() {
                return Err(format!(
                    "Expected a number at 4th word, line at {}: {}",
                    i,
                    std::str::from_utf8(l[3]).unwrap()
                ));
            }
            if let Err(_) = std::str::from_utf8(l[6]).unwrap().parse::<usize>() {
                return Err(format!(
                    "Expected a number at 6th word, line at {}: {}",
                    i,
                    std::str::from_utf8(l[3]).unwrap()
                ));
            }
            if let Err(_) = std::str::from_utf8(l[13]).unwrap().parse::<usize>() {
                return Err(format!(
                    "Expected a number at 6th word, line at {}: {}",
                    i,
                    std::str::from_utf8(l[3]).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let competitors = Self::competitors(data);
        let mut champion = 0;
        for c in competitors {
            champion = champion.max(c.run(TIME));
        }
        Solution::new("", champion.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut competitors = Self::competitors(data);
        for _ in 0..TIME {
            let mut lead = 0;
            for c in competitors.iter_mut() {
                lead = lead.max(c.tick());
            }
            for c in competitors.iter_mut() {
                c.reward(lead);
            }
        }

        let mut champion = 0;
        for c in competitors.iter_mut() {
            champion = champion.max(c.points);
        }
        Solution::new("The champion reindeer has run:", champion.to_string())
    }
}

struct ReinDeer {
    speed: usize,
    duration: usize,
    rest: usize,
    points: usize,
    run: usize,
    state: State,
}

enum State {
    Running(usize),
    Resting(usize),
}

impl ReinDeer {
    fn parse(data: &[u8]) -> Self {
        let mut iter = data.split(|c| c.is_ascii_whitespace());
        let speed = std::str::from_utf8(iter.nth(3).expect("Speed not found"))
            .expect("Speed is not a number")
            .parse()
            .expect("Speed is not a number");
        let duration = std::str::from_utf8(iter.nth(2).expect("Duration not found"))
            .expect("Duration is not a number")
            .parse()
            .expect("Duration is not a number");
        let rest = std::str::from_utf8(iter.nth(6).expect("Rest time not found"))
            .expect("Rest time is not a number")
            .parse()
            .expect("Rest time is not a number");
        ReinDeer {
            speed,
            duration,
            rest,
            points: 0,
            run: 0,
            state: State::Running(duration),
        }
    }
    #[inline]
    fn reward(&mut self, distance: usize) {
        if self.run == distance {
            self.points += 1;
        }
    }

    #[inline]
    fn tick(&mut self) -> usize {
        match self.state {
            State::Running(ref mut r) => {
                *r -= 1;
                if *r == 0 {
                    self.state = State::Resting(self.rest);
                }
                self.run += self.speed;
            }
            State::Resting(ref mut r) => {
                *r -= 1;
                if *r == 0 {
                    self.state = State::Running(self.duration);
                }
            }
        }
        self.run
    }

    fn run(&self, mut time: usize) -> usize {
        let mut run = 0;
        while time > 0 {
            let drtn = self.duration.min(time);
            run += self.speed * drtn;
            time -= drtn;
            time -= self.rest.min(time);
        }
        run
    }
}

impl D14 {
    fn competitors(data: Vec<u8>) -> Vec<ReinDeer> {
        let mut reindeers = Vec::with_capacity(8);
        for line in data.split(|&c| c == b'\n') {
            reindeers.push(ReinDeer::parse(line));
        }
        reindeers
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    #[test]
    fn test_part_one() {
        let solver = super::D14;
        let data = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#
            .as_bytes()
            .to_vec();
        // change const TIME = 1000 to test
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "1120");
    }

    #[test]
    fn test_part_two() {
        let solver = super::D14;
        let data = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#
            .as_bytes()
            .to_vec();
        // change const TIME = 1000 to test
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "689");
    }
}
