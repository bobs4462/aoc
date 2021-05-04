/// --- Day 18: Like a GIF For Your Yard ---
pub struct D18;

use crate::solver::{Solution, Solver};

impl Solver for D18 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            if !l.iter().all(|&c| c == b'#' || c == b'.') {
                return Err(format!("Wrong line:\n{}", std::str::from_utf8(l).unwrap()));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut animator = Animator::new(data, false);
        for _ in 0..100 {
            animator.next();
        }
        let state = if animator.state == 1 {
            &animator.state1
        } else {
            &animator.state2
        };
        let lights_on = state.iter().fold(0, |acc, &l| acc + (l == 1) as usize);
        Solution::new("Number of lights on is: ", lights_on.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut animator = Animator::new(data, true);
        for _ in 0..100 {
            animator.next();
        }
        let state = if animator.state == 1 {
            &animator.state1
        } else {
            &animator.state2
        };
        let lights_on = state.iter().fold(0, |acc, &l| acc + (l == 1) as usize);
        Solution::new("Number of lights on is: ", lights_on.to_string())
    }
}

struct Animator {
    state1: Vec<u8>,
    state2: Vec<u8>,
    state: usize,
    side: usize,
    stuck: bool,
}

impl Animator {
    fn new(input: Vec<u8>, stuck: bool) -> Self {
        let mut state1 = Vec::with_capacity(input.len());
        let state2 = vec![0u8; input.len()];
        let lines = input.split(|&c| c == b'\n');
        let mut side: usize = 0;
        for l in lines {
            side += 1;
            for &c in l {
                if c == b'#' {
                    state1.push(1);
                } else {
                    state1.push(0)
                }
            }
        }
        if stuck {
            state1[0] = 1;
            state1[side - 1] = 1;
            state1[side * side - side] = 1;
            state1[side * side - 1] = 1;
        }
        Animator {
            state1,
            state2,
            state: 1,
            side,
            stuck,
        }
    }

    fn next(&mut self) {
        let (next, state) = if self.state == 1 {
            self.state = 2;
            (&mut self.state2, &mut self.state1)
        } else {
            self.state = 1;
            (&mut self.state1, &mut self.state2)
        };

        for i in 0..self.side {
            for j in 0..self.side {
                let target = i * self.side + j;
                let curr = unsafe { *state.get_unchecked(target) };
                // 1
                let mut on = if i != 0 && j != 0 {
                    let index = target - self.side - 1;
                    if index == 0 {
                        1
                    } else {
                        unsafe { *state.get_unchecked(index) }
                    }
                } else {
                    0
                };
                // 2
                on += if i != 0 {
                    let index = target - self.side;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 3
                on += if i != 0 && j != self.side - 1 {
                    let index = target - self.side + 1;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 4
                on += if j != 0 {
                    let index = target - 1;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 5
                on += if j != self.side - 1 {
                    let index = target + 1;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 6
                on += if i != self.side - 1 && j != 0 {
                    let index = target + self.side - 1;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 7
                on += if i != self.side - 1 {
                    let index = target + self.side;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // 8
                on += if i != self.side - 1 && j != self.side - 1 {
                    let index = target + self.side + 1;
                    unsafe { *state.get_unchecked(index) }
                } else {
                    0
                };
                // TODO simplify
                let corner = self.stuck
                    && (target == 0
                        || target == self.side - 1
                        || target == self.side * self.side - 1
                        || target == self.side * self.side - self.side);

                next[target] = (corner || (on == 2 && curr == 1) || on == 3) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Solver, D18};
    #[test]
    fn test_part_one() {
        let data = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#
            .to_string()
            .into_bytes();

        let solver = D18;
        // Change steps from 100 to 4 to test
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "4");
    }

    #[test]
    fn test_part_two() {
        let data = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#
            .to_string()
            .into_bytes();

        let solver = D18;
        // Change steps from 100 to 5 to test
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "17");
    }
}
