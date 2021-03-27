/// --- Day 12: JSAbacusFramework.io ---
pub struct D12;

use crate::solver::{Solution, Solver};

const ZERO: u8 = 0x30;
impl Solver for D12 {
    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let mut total = 0;
        let mut seq = data.into_iter();
        let mut negative = 1;
        while let Some(c) = seq.next() {
            if c.is_ascii_digit() {
                let mut builder = (c - ZERO) as i32;
                while let Some(c) = seq.next() {
                    if c.is_ascii_digit() {
                        builder = builder * 10 + (c - ZERO) as i32;
                    } else {
                        total += builder * negative;
                        negative = 1;
                        break;
                    }
                }
            } else if c == b'-' {
                negative = -1;
            }
        }
        Solution::new("Sum of all numbers is", total.to_string())
    }

    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let mut collections: Vec<Collection> = Vec::with_capacity(20);

        let mut seq = data.into_iter().peekable();

        let mut negative: i32 = 1;
        let mut red: usize = 0;
        let mut total: i32 = 0;
        while let Some(c) = seq.next() {
            match c {
                b'{' => {
                    collections.push(Collection::Object(0));
                }
                b'}' => {
                    if let Some(Collection::Object(ref c)) = collections.pop() {
                        match collections.last_mut() {
                            Some(Collection::Object(ref mut p))
                            | Some(Collection::Array(ref mut p)) => *p += *c,
                            None => total = *c,
                        }
                    }
                }
                b'[' => {
                    collections.push(Collection::Array(0));
                }
                b']' => {
                    if let Some(Collection::Array(ref c)) = collections.pop() {
                        match collections.last_mut() {
                            Some(Collection::Object(ref mut p))
                            | Some(Collection::Array(ref mut p)) => *p += *c,
                            None => total = *c,
                        }
                    }
                }
                b'r' | b'e' => {
                    red += 1;
                }
                b'd' => {
                    if red == 2 {
                        if let Some(Collection::Array(_)) = collections.last() {
                            red = 0;
                            continue;
                        }
                        let mut level = 0;
                        while let Some(c) = seq.next() {
                            match c {
                                b'{' => level += 1,
                                b'}' => {
                                    if level > 0 {
                                        level -= 1;
                                        continue;
                                    }
                                    collections.pop();
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                0x30..=0x39 => {
                    let mut builder = (c - ZERO) as i32;
                    while let Some(&c) = seq.peek() {
                        if c.is_ascii_digit() {
                            builder = builder * 10 + (c - ZERO) as i32;
                            seq.next();
                        } else {
                            if let Some(Collection::Object(ref mut v))
                            | Some(Collection::Array(ref mut v)) = collections.last_mut()
                            {
                                *v += builder * negative;
                                negative = 1;
                                break;
                            }
                        }
                    }
                }
                b'-' => negative = -1,
                _ => {
                    red = 0;
                }
            }
        }

        Solution::new("The solution is", total.to_string())
    }
}

enum Collection {
    Array(i32),
    Object(i32),
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_part_one() {
        let solver = super::D12;
        let data = b"[1,2,3]".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "6");
        let data = b"{\"a\":2,\"b\":4}".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "6");
        let data = b"{\"a\":{\"b\":4},\"c\":-1}".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "3");
        let data = b"[-1,{\"a\":1}]".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "0");
        let data = b"{\"a\":[-1,1]}".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "0");
        let data = b"{}".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "0");
        let data = r#"
{"e":[[{"e":86,"c":23,"a":{"a":[120,169,"green","red","orange"],"b":"red"},"g":"yellow","b":["yellow"],"d":"red","f":-19},{"e":-47,"a":[2],"d":{"a":"violet"},"c"
    "#.to_string().into();

        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "334");
    }
    #[test]
    fn test_part_two() {
        let solver = super::D12;
        let data = b"[1,2,3]".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "6");
        let data = b"[1,{\"c\":\"red\",\"b\":2},3]".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "4");
        let data = b"{\"a\":{\"b\":4},\"c\":-1}".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "3");
        let data = b"{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "0");
        let data = b"{\"a\":[-1,1]}".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "0");
        let data = b"[1,\"red\",5]".to_vec();
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "6");
    }
}
