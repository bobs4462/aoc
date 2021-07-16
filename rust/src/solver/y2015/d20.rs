/// --- Day 20: Infinite Elves and Infinite Houses ---
pub struct D20;

use crate::solver::{Solution, Solver};

impl Solver for D20 {
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }
}

fn rho(mut composite: isize) -> Vec<isize> {
    let mut res = Vec::new();
    let mut size = 2;

    let mut a = 2;
    let mut b = 2;
    'outer: loop {
        loop {
            a = random(a, composite);
            println!("A: {}, B: {}", a, b);
            if size == 0 {
                size *= 2;
                b = a;
                continue 'outer;
            }

            if a == 0 {
                println!("BREAKING");
                break 'outer;
            }
            if gcd((a - b).abs(), composite) > 1 {
                let factor = gcd((a - b).abs(), composite);
                println!("Factor: {}", factor);
                res.push((composite / factor).min(factor));
                if factor == composite {
                    return res;
                } else {
                    composite = (composite / factor).max(factor);
                    continue 'outer;
                }
            }
            size -= 1;
        }
    }
    res
}

#[inline]
fn random(x: isize, n: isize) -> isize {
    (x.pow(2) + 1) % n
}

#[inline]
fn gcd(mut a: isize, mut b: isize) -> isize {
    println!("GCD FOR: {} AND {}", a, b);
    while a % b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    b
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rho() {
        let res = super::rho(5);
        assert_eq!(res, vec![5]);
    }
}
