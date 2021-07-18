/// --- Day 20: Infinite Elves and Infinite Houses ---
pub struct D20;

use std::collections::HashMap;

use crate::solver::{Solution, Solver};

impl Solver for D20 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for c in input {
            if !c.is_ascii_digit() {
                return Err(format!(
                    "Wrong input:\n{}",
                    std::str::from_utf8(input).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let target = std::str::from_utf8(&data)
            .unwrap()
            .parse::<isize>()
            .unwrap();

        let mut house = 2;
        let mut factorizer = Factors::new();

        let mut factors;
        let mut presents;

        loop {
            factors = factorizer.td(house);
            presents = factors.iter().fold(0, |acc, n| acc + n * 10);
            if presents + 10 >= target {
                break;
            }
            house += 1;
        }

        Solution::new("The house number", house.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }
}

struct Factors {
    memoized: HashMap<isize, Vec<isize>>,
    primes: Vec<isize>,
}

impl Factors {
    fn new() -> Self {
        let mut memoized = HashMap::new();
        memoized.insert(2, vec![2]);

        Factors {
            memoized,
            primes: vec![2, 3, 5, 7, 11],
        }
    }

    fn td(&mut self, mut number: isize) -> &Vec<isize> {
        let mut primes = self.primes.iter();
        let mut factor = *primes.next().unwrap();
        let key = number;

        if self.memoized.contains_key(&key) {
            return self.memoized.get(&key).unwrap();
        }

        'outer: while number != 1 {
            while number % factor != 0 {
                if let Some(&p) = primes.next() {
                    factor = p;
                } else {
                    factor += 2;
                }
            }
            number /= factor;
            self.memoized.entry(key).or_insert(Vec::new()).push(factor);

            if self.memoized.contains_key(&number) {
                let mut existing = self.memoized.get(&number).unwrap().clone();
                self.memoized.get_mut(&key).unwrap().append(&mut existing);

                break 'outer;
            }
        }
        let res = self.memoized.get(&key).unwrap();
        if res.len() == 1 {
            self.primes.push(*res.first().unwrap());
        }
        self.memoized.get_mut(&key).unwrap().push(key);
        self.memoized.get_mut(&key).unwrap().sort();
        self.memoized.get_mut(&key).unwrap().dedup();

        self.memoized.get(&key).unwrap()
    }
}

#[inline]
fn gcd(mut a: isize, mut b: isize) -> isize {
    while a % b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;

    #[test]
    fn test_td() {
        let mut f = super::Factors::new();
        let mut res = f.td(2);
        assert_eq!(res, &vec![2]);
        res = f.td(4);
        assert_eq!(res, &vec![2, 4]);
        res = f.td(8);
        assert_eq!(res, &vec![2, 4, 8]);
        let mut res = f.td(16);
        assert_eq!(res, &vec![2, 4, 8, 16]);
        res = f.td(32);
        assert_eq!(res, &vec![2, 4, 8, 16, 32]);
        res = f.td(80);
        assert_eq!(res, &vec![2, 5, 80]);
        res = f.td(113);
        assert_eq!(res, &vec![113]);
        res = f.td(226);
        assert_eq!(res, &vec![2, 113, 226]);
    }

    #[test]
    fn test_part_one() {
        let solver = super::D20;
        let data = b"70".to_vec();
        let res = solver.solve_part_one(data);

        assert_eq!(res.value, "4");
    }
}
