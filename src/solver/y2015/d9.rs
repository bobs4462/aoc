/// --- Day 9: All in a Single Night ---
pub struct D9;

use crate::solver::{Solution, Solver};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

struct Node<'a> {
    name: &'a str,
    paths: Vec<Path<'a>>,
}

type RefNode<'a> = Rc<RefCell<Node<'a>>>;

struct Runner<'a> {
    current: RefNode<'a>,
    total: usize,
    visited: HashSet<&'a str>,
}

struct Path<'a> {
    dst: RefNode<'a>,
    weigth: usize,
}

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
        let shortest = walk(data, usize::min, usize::MAX);
        Solution::new("Shortest path is:", shortest.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let longest = walk(data, usize::max, usize::MIN);
        Solution::new("Longest path is:", longest.to_string())
    }
}

fn walk<F>(data: Vec<u8>, cmp: F, start: usize) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let lines = data.split(|&c| c == b'\n');
    let mut nodes: HashMap<&str, RefNode> = HashMap::new();
    for l in lines {
        let path = unsafe { std::str::from_utf8_unchecked(l) };
        let path: Vec<&str> = path.split(' ').collect();

        let weigth = path[4].parse().unwrap();
        let source: RefNode;
        let destination: RefNode;
        if let Some(src) = nodes.get(path[0]) {
            source = Rc::clone(src);
            if let Some(dst) = nodes.get(path[2]) {
                destination = Rc::clone(dst);
            } else {
                destination = Rc::new(RefCell::new(Node {
                    name: path[2],
                    paths: Vec::new(),
                }));
            }
        } else {
            if let Some(dst) = nodes.get(path[2]) {
                destination = Rc::clone(dst);
            } else {
                destination = Rc::new(RefCell::new(Node {
                    name: path[2],
                    paths: Vec::new(),
                }));
            }
            source = Rc::new(RefCell::new(Node {
                name: path[0],
                paths: Vec::new(),
            }));
        }
        source.borrow_mut().paths.push(Path {
            dst: Rc::clone(&destination),
            weigth,
        });
        destination.borrow_mut().paths.push(Path {
            dst: Rc::clone(&source),
            weigth,
        });
        nodes.insert(path[0], source);
        nodes.insert(path[2], destination);
    }
    let count = nodes.len();
    let mut val = start;
    for (_, v) in nodes {
        let mut tovisit = Vec::new();
        tovisit.push(Runner {
            current: Rc::clone(&v),
            total: 0,
            visited: HashSet::new(),
        });
        while !tovisit.is_empty() {
            let mut r = tovisit.pop().unwrap();
            r.visited.insert(r.current.borrow().name);

            if r.visited.len() == count {
                val = cmp(val, r.total);
                continue;
            }

            for p in r.current.borrow().paths.iter() {
                if r.visited.contains(p.dst.borrow().name) {
                    continue;
                }
                tovisit.push(Runner {
                    current: Rc::clone(&p.dst),
                    total: r.total + p.weigth,
                    visited: r.visited.clone(),
                });
            }
        }
    }
    val
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let solver = super::D9;
        let data =
            b"London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "605");
        let data = r#"Faerun to Norrath = 129
Faerun to Tristram = 58
Faerun to AlphaCentauri = 13
Faerun to Arbre = 24
Faerun to Snowdin = 60
Faerun to Tambi = 71
Faerun to Straylight = 67
Norrath to Tristram = 142
Norrath to AlphaCentauri = 15
Norrath to Arbre = 135
Norrath to Snowdin = 75
Norrath to Tambi = 82
Norrath to Straylight = 54
Tristram to AlphaCentauri = 118
Tristram to Arbre = 122
Tristram to Snowdin = 103
Tristram to Tambi = 49
Tristram to Straylight = 97
AlphaCentauri to Arbre = 116
AlphaCentauri to Snowdin = 12
AlphaCentauri to Tambi = 18
AlphaCentauri to Straylight = 91
Arbre to Snowdin = 129
Arbre to Tambi = 53
Arbre to Straylight = 40
Snowdin to Tambi = 15
Snowdin to Straylight = 99
Tambi to Straylight = 70"#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "207");
    }
    #[test]
    fn test_part_two() {
        let solver = super::D9;
        let data =
            b"London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "982");
        let data = r#"Faerun to Norrath = 129
Faerun to Tristram = 58
Faerun to AlphaCentauri = 13
Faerun to Arbre = 24
Faerun to Snowdin = 60
Faerun to Tambi = 71
Faerun to Straylight = 67
Norrath to Tristram = 142
Norrath to AlphaCentauri = 15
Norrath to Arbre = 135
Norrath to Snowdin = 75
Norrath to Tambi = 82
Norrath to Straylight = 54
Tristram to AlphaCentauri = 118
Tristram to Arbre = 122
Tristram to Snowdin = 103
Tristram to Tambi = 49
Tristram to Straylight = 97
AlphaCentauri to Arbre = 116
AlphaCentauri to Snowdin = 12
AlphaCentauri to Tambi = 18
AlphaCentauri to Straylight = 91
Arbre to Snowdin = 129
Arbre to Tambi = 53
Arbre to Straylight = 40
Snowdin to Tambi = 15
Snowdin to Straylight = 99
Tambi to Straylight = 70"#
            .as_bytes()
            .to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "804");
    }
    #[test]
    fn test_permutator() {
        use super::Permutator;
        let vec = &mut ["Boburbek", "Makhmudov", "Nodirbekovich"];
        let permutator = Permutator::new(vec);
        for p in permutator {
            println!("{:?}", p);
        }
    }
}

/// Permuation generator, using Narayana’s next single permutation algorithm
struct Permutator<'a, T: PartialEq + Ord + 'a> {
    // Actually I just wrote it for fun :)
    collection: *mut T,
    len: usize,
    permutaions: u128,
    factorial: u128,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: PartialEq + Ord> Permutator<'a, T> {
    fn new(collection: &'a mut [T]) -> Self {
        collection.sort();
        let factorial = fact(collection.len() as u8);
        Permutator {
            collection: collection.as_mut_ptr(),
            len: collection.len(),
            permutaions: 0,
            factorial,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn done(&self) -> bool {
        self.permutaions == self.factorial
    }
}
impl<'a, T: PartialEq + Ord> Iterator for Permutator<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.done() {
            return None;
        }
        let slice = unsafe { std::slice::from_raw_parts_mut(self.collection, self.len) };
        let iter = slice.windows(2).enumerate();
        let mut k: usize = 0;
        let mut j: usize = 0;
        for (i, p) in iter {
            if p[0] < p[1] {
                k = i;
            }
        }
        let iter = slice[k + 1..].iter().enumerate();
        for (i, t) in iter {
            if slice[k] < *t {
                j = i + k + 1;
            }
        }
        slice.swap(k, j);
        slice[k + 1..].reverse();
        self.permutaions += 1;
        Some(slice)
    }
}

fn fact(val: u8) -> u128 {
    let mut res = 1;
    for i in 1..=val {
        res *= i as u128;
    }
    res
}
