/// --- Day 9: All in a Single Night ---
pub struct D9;

use crate::solver::{Solution, Solver};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

impl Solver for D9 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        let lines = input.split(|&c| c == b'\n');
        for (i, l) in lines.enumerate() {
            if l.split(|&c| c == b' ').count() != 5 {
                return Err(format!(
                    "Line {} is invalid: {}",
                    i,
                    std::str::from_utf8(l).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|&c| c == b'\n');
        let mut nodes: HashMap<&str, RefNode> = HashMap::new();
        for l in lines {
            let path = unsafe { std::str::from_utf8_unchecked(l) };
            let path: Vec<&str> = path.split(' ').collect();

            let weigth = path[5].parse().unwrap();
            if let Some(src) = nodes.get(path[0]) {
                if let Some(dst) = nodes.get(path[3]) {
                    src.borrow_mut().paths.push(Path {
                        dst: Rc::clone(dst),
                        weigth,
                    });
                } else {
                    let node = Rc::new(RefCell::new(Node {
                        name: path[3],
                        paths: Vec::new(),
                    }));
                    src.borrow_mut().paths.push(Path {
                        dst: Rc::clone(&node),
                        weigth,
                    });
                    nodes.insert(path[3], node);
                }
            } else {
                let mut paths = Vec::new();
                if let Some(dst) = nodes.get(path[3]) {
                    paths.push(Path {
                        dst: Rc::clone(dst),
                        weigth,
                    });
                } else {
                    let node = Rc::new(RefCell::new(Node {
                        name: path[3],
                        paths: Vec::new(),
                    }));
                    paths.push(Path {
                        dst: Rc::clone(&node),
                        weigth,
                    });
                    nodes.insert(path[3], node);
                }
                let src = Rc::new(RefCell::new(Node {
                    name: path[0],
                    paths,
                }));
                nodes.insert(path[0], src);
            }
        }
        let count = nodes.len();
        for (k, v) in nodes {
            let mut jumps = 0;
            let mut distance = 0;
            let mut visited = HashSet::new();
            let mut tovisit = Vec::new();
            visited.insert(k);
            tovisit.push(v);
            while !tovisit.is_empty() {
                let next = tovisit.pop().unwrap();
                for p in next.borrow().paths.iter() {
                    if visited.contains(p.dst.borrow().name) {
                        continue;
                    }
                    tovisit.push(Rc::clone(&p.dst));
                }
            }
        }
        Solution::new("", "".to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", "".to_string())
    }
}

struct Node<'a> {
    name: &'a str,
    paths: Vec<Path<'a>>,
}

type RefNode<'a> = Rc<RefCell<Node<'a>>>;

struct Path<'a> {
    dst: RefNode<'a>,
    weigth: usize,
}
