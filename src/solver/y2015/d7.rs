/// --- Day 7: Some Assembly Required ---
pub struct D7;

use crate::solver::{Solution, Solver};
use std::collections::{HashMap, HashSet};

struct Expr {
    output: String,
    deps: Vec<String>,
    input: String,
}

impl Expr {
    fn parse(expr: &str) -> Self {
        let mut split = expr.split(" -> ");
        let mut deps: Vec<String> = Vec::new();
        let input = split.next().unwrap().to_string();
        let expr: Vec<&str> = input.split(' ').collect();
        let output = split.next().unwrap().to_string();
        match expr.len() {
            1 => {
                if let Err(_) = expr[0].parse::<u16>() {
                    deps.push(expr[0].into());
                }
            }
            2 => {
                if let Err(_) = expr[1].parse::<u16>() {
                    deps.push(expr[1].into());
                }
            }
            3 => {
                if let Err(_) = expr[0].parse::<u16>() {
                    deps.push(expr[0].into());
                }
                if let Err(_) = expr[2].parse::<u16>() {
                    deps.push(expr[2].into());
                }
            }
            _ => panic!("The length of input is more than 3!"),
        }
        Expr {
            output,
            deps,
            input,
        }
    }
}

impl Solver for D7 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for l in lines {
            let mut split = l.split(|&c| c == b' ').rev();
            if split.nth(1).unwrap() != &[b'-', b'>'] {
                return Err(format!(
                    "Invalid line in the input: {}",
                    std::str::from_utf8(l).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|&c| c == b'\n');
        let mut expressions = Vec::with_capacity(350);
        for l in lines {
            expressions.push(Expr::parse(unsafe { std::str::from_utf8_unchecked(l) }))
        }
        let tsorted = tsort(&expressions);
        let mut resolved: HashMap<&str, u16> = HashMap::with_capacity(350);
        compute(&tsorted, &mut resolved);
        Solution::new("", resolved.get("a").unwrap().to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|&c| c == b'\n');
        let mut expressions = Vec::with_capacity(350);
        for l in lines {
            expressions.push(Expr::parse(unsafe { std::str::from_utf8_unchecked(l) }))
        }
        let tsorted = tsort(&expressions);
        let mut resolved: HashMap<&str, u16> = HashMap::with_capacity(350);
        compute(&tsorted, &mut resolved);
        let a = *resolved.get("a").unwrap();
        resolved.clear();
        resolved.insert("b", a);
        compute(&tsorted, &mut resolved);
        Solution::new("", resolved.get("a").unwrap().to_string())
    }
}

fn binary(lhs: &str, rhs: &str, resolved: &HashMap<&str, u16>) -> (u16, u16) {
    let l = if let Ok(v) = lhs.parse::<u16>() {
        v
    } else {
        *resolved.get(lhs).unwrap()
    };
    let r = if let Ok(v) = rhs.parse::<u16>() {
        v
    } else {
        *resolved.get(rhs).unwrap()
    };
    (l, r)
}

fn compute<'a, 'b: 'a>(tsorted: &'a Vec<&'b Expr>, resolved: &'a mut HashMap<&'b str, u16>) {
    for e in tsorted.iter() {
        if resolved.contains_key(e.output.as_str()) {
            continue;
        }
        let parts = e.input.split(' ').collect::<Vec<&str>>();
        match parts.len() {
            1 => {
                if let Ok(v) = parts[0].parse::<u16>() {
                    resolved.insert(&e.output, v);
                } else {
                    resolved.insert(&e.output, *resolved.get(parts[0]).unwrap());
                }
            }
            2 => {
                if let Ok(v) = parts[1].parse::<u16>() {
                    resolved.insert(&e.output, !v);
                } else {
                    resolved.insert(&e.output, !*resolved.get(parts[1]).unwrap());
                }
            }
            3 => {
                let (lhs, rhs) = binary(parts[0], parts[2], &resolved);
                match parts[1] {
                    "AND" => resolved.insert(&e.output, lhs & rhs),
                    "OR" => resolved.insert(&e.output, lhs | rhs),
                    "LSHIFT" => resolved.insert(&e.output, lhs << rhs),
                    "RSHIFT" => resolved.insert(&e.output, lhs >> rhs),
                    _ => panic!("Incorrect Gate!: {}", parts[1]),
                };
            }
            _ => panic!("Incorrect length of input!"),
        }
    }
}

fn tsort<'a>(expressions: &'a Vec<Expr>) -> Vec<&'a Expr> {
    let mut tsorted = Vec::with_capacity(350);
    let mut resolved: HashSet<&String> = HashSet::with_capacity(350);
    while tsorted.len() != expressions.len() {
        'e: for e in expressions.iter() {
            if resolved.contains(&e.output) {
                continue;
            }
            for d in e.deps.iter() {
                if !resolved.contains(d) {
                    continue 'e;
                }
            }
            tsorted.push(e);
            resolved.insert(&e.output);
        }
    }
    tsorted
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let solver = super::D7;
        let data = b"123 -> x\n456 -> y\nx AND y -> a\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "72");
    }
}
