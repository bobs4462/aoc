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
        let mut objects: Vec<usize> = Vec::with_capacity(20);

        let mut arrays: Vec<bool> = Vec::with_capacity(20);

        let mut seq = data.into_iter();

        let mut negative = 1;
        let mut total = 0;
        let mut inarray = false;
        while let Some(c) = seq.next() {
            match c {
                b'{' => {
                    objects.push(0);
                }
                b'}' => {}
                b'[' => inarray = true,
                b']' => inarray = false,
                0x30..=0x39 => {
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
                }
                b'-' => negative = -1,
                _ => {}
            }
        }

        Solution::new("The solution is", total.to_string())
    }
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
}
