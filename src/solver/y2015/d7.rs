/// --- Day 7: Some Assembly Required ---
pub struct D7;

use std::collections::HashMap;

use crate::solver::{Solution, Solver};

#[derive(Debug)]
struct Deps<'a> {
    storage: HashMap<&'a str, u16>,
}

impl<'a> Deps<'a> {
    fn new() -> Self {
        Deps {
            storage: HashMap::with_capacity(350),
        }
    }
    fn satisfies(&self, dep: Dependency) -> bool {
        match dep {
            Dependency::Binary(lhs, rhs) => {
                self.storage.contains_key(lhs) && self.storage.contains_key(rhs)
            }
            Dependency::Unary(val) => self.storage.contains_key(val),
            Dependency::Empty => true,
        }
    }
    fn add(&mut self, key: &'a str, val: u16) {
        self.storage.insert(key, val);
    }

    fn get(&self, key: &str) -> u16 {
        *self.storage.get(key).expect("Key does not exist")
    }

    fn solved(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }
}

struct Expr {
    output: String,
    input: Input,
}

impl Expr {
    fn parse(expr: &str) -> Self {
        let mut split = expr.split(" -> ");
        // println!("SPLIT: {:?}", split);
        let expr: Vec<&str> = split.next().unwrap().split(' ').collect();
        let output = String::from(split.next().unwrap());
        let input: Input;
        match expr.len() {
            1 => input = Input::ident(expr[0]),
            2 => input = Input::not(expr[1]),
            3 => input = Input::binary(expr[0], expr[1], expr[2]),
            l => panic!("Wrong length of expr tokens list: {}", l),
        }
        Expr { output, input }
    }

    fn dependency(&self) -> Dependency {
        match self.input {
            Input::And(Operand::Wire(ref lhs), Operand::Wire(ref rhs)) => {
                Dependency::Binary(lhs, rhs)
            }
            Input::And(Operand::Val(_), Operand::Wire(ref rhs)) => Dependency::Unary(rhs),
            Input::And(Operand::Wire(ref lhs), Operand::Val(_)) => Dependency::Unary(lhs),

            Input::Or(Operand::Wire(ref lhs), Operand::Wire(ref rhs)) => {
                Dependency::Binary(lhs, rhs)
            }
            Input::Or(Operand::Wire(ref lhs), Operand::Val(_)) => Dependency::Unary(lhs),
            Input::Or(Operand::Val(_), Operand::Wire(ref rhs)) => Dependency::Unary(rhs),

            Input::LShift(Operand::Wire(ref lhs), Operand::Wire(ref rhs)) => {
                Dependency::Binary(lhs, rhs)
            }
            Input::LShift(Operand::Wire(ref lhs), Operand::Val(_)) => Dependency::Unary(lhs),
            Input::LShift(Operand::Val(_), Operand::Wire(ref rhs)) => Dependency::Unary(rhs),

            Input::RShift(Operand::Wire(ref lhs), Operand::Wire(ref rhs)) => {
                Dependency::Binary(lhs, rhs)
            }
            Input::RShift(Operand::Wire(ref lhs), Operand::Val(_)) => Dependency::Unary(lhs),
            Input::RShift(Operand::Val(_), Operand::Wire(ref rhs)) => Dependency::Unary(rhs),

            Input::Ident(Operand::Wire(ref val)) => Dependency::Unary(val),

            Input::Not(Operand::Wire(ref val)) => Dependency::Unary(val),
            _ => Dependency::Empty,
        }
    }
}

#[derive(Debug)]
enum Dependency<'a> {
    Binary(&'a str, &'a str),
    Unary(&'a str),
    Empty,
}

enum Operand {
    Wire(String),
    Val(u16),
}

impl Operand {
    fn parse(expr: &str) -> Self {
        if let Ok(val) = expr.parse::<u16>() {
            Operand::Val(val)
        } else {
            Operand::Wire(String::from(expr))
        }
    }
}

enum Input {
    Ident(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
    Not(Operand),
}

impl Input {
    fn binary(lhs: &str, op: &str, rhs: &str) -> Self {
        let (lhs, rhs) = (Operand::parse(lhs), Operand::parse(rhs));
        match op {
            "AND" => Self::And(lhs, rhs),
            "OR" => Self::Or(lhs, rhs),
            "RSHIFT" => Self::RShift(lhs, rhs),
            "LSHIFT" => Self::LShift(lhs, rhs),
            o => panic!("Wrong binary operator: {}", o),
        }
    }
    fn not(val: &str) -> Self {
        Self::Not(Operand::parse(val))
    }
    fn ident(val: &str) -> Self {
        Self::Ident(Operand::parse(val))
    }
}

impl Solver for D7 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for (i, l) in lines.enumerate() {
            let s;
            unsafe {
                s = std::str::from_utf8_unchecked(l);
            }
            if !s.contains("->") {
                return Err(format!("Line {} is invalid: {}", i, s));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut deps = Deps::new();
        let exprs = self.expressions(data.as_slice());
        while !deps.solved("a") {
            for e in exprs.iter() {
                if deps.solved(&e.output) {
                    continue;
                }
                if !deps.satisfies(e.dependency()) {
                    // println!("DEPS: {:?}, REQUEST: {:?}", deps, e.dependency());
                    continue;
                }
                match e.input {
                    Input::Ident(ref val) => match val {
                        Operand::Val(v) => deps.add(&e.output, *v),
                        Operand::Wire(ref w) => deps.add(&e.output, deps.get(w)),
                    },
                    Input::Not(ref val) => match val {
                        Operand::Val(v) => deps.add(&e.output, !(*v)),
                        Operand::Wire(ref w) => deps.add(&e.output, !deps.get(w)),
                    },
                    Input::And(ref lhs, ref rhs) => {
                        let (l, r) = self.binary(&deps, lhs, rhs);
                        deps.add(&e.output, l & r);
                    }
                    Input::Or(ref lhs, ref rhs) => {
                        let (l, r) = self.binary(&deps, lhs, rhs);
                        deps.add(&e.output, l | r);
                    }
                    Input::LShift(ref lhs, ref rhs) => {
                        let (l, r) = self.binary(&deps, lhs, rhs);
                        deps.add(&e.output, l << r);
                    }
                    Input::RShift(ref lhs, ref rhs) => {
                        let (l, r) = self.binary(&deps, lhs, rhs);
                        deps.add(&e.output, l >> r);
                    }
                }
            }
        }
        Solution::new("The 'a' wire has the output of:", deps.get("a").to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("The 'a' wire has the output of:", "a".to_string())
    }
}

impl D7 {
    fn expressions(&self, data: &[u8]) -> Vec<Expr> {
        let lines = data.split(|&c| c == b'\n');
        let mut result = Vec::with_capacity(350);
        for l in lines {
            let s;
            unsafe {
                s = std::str::from_utf8_unchecked(l);
            }
            result.push(Expr::parse(s));
        }
        result
    }

    fn binary(&self, deps: &Deps, lhs: &Operand, rhs: &Operand) -> (u16, u16) {
        let l: u16;
        let r: u16;
        match lhs {
            Operand::Val(v) => l = *v,
            Operand::Wire(ref w) => l = deps.get(w),
        }
        match rhs {
            Operand::Val(v) => r = *v,
            Operand::Wire(ref w) => r = deps.get(w),
        }
        (l, r)
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let data = b"123 -> x\n456 -> y\nx AND y -> a\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i".to_vec();
        let solver = super::D7;
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "72");
    }
}
