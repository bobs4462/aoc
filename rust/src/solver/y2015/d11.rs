/// --- Day 11: Corporate Policy ---
/// There could be some distinct patterns in input, which can be easily solved analytically:
/// "l" - is some randome letter
/// 1. ZYXlllll, where Z, Y, X are distinct letters, which are not in incrementing sequence
/// 2. XYZlllll, where X, Y, Z are distinct letters, which are in incrementing sequence
/// 3. XXllllll AND lXXlllll, where XX is a pair of allowed letters
/// 4. XXYZllll AND XYZZllll
pub struct D11;

use crate::solver::{Solution, Solver};

impl Solver for D11 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for &c in input {
            if !c.is_ascii_alphabetic() {
                return Err(format!(
                    "The character is not an ASCII digit: {}",
                    std::char::from_u32(c as u32).unwrap()
                ));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, mut data: Vec<u8>) -> Solution {
        for i in data.iter_mut() {
            *i -= 0x61;
        }
        while Self::wrong(&mut data) {
            Self::increment(&mut data);
        }
        for i in data.iter_mut() {
            *i += 0x61;
        }
        Solution::new("The solution is", String::from_utf8(data).unwrap())
    }

    fn solve_part_two(&self, mut data: Vec<u8>) -> Solution {
        for i in data.iter_mut() {
            *i -= 0x61;
        }
        while Self::wrong(&mut data) {
            Self::increment(&mut data);
        }
        data[7] += 1;
        while Self::wrong(&mut data) {
            Self::increment(&mut data);
        }
        for i in data.iter_mut() {
            *i += 0x61;
        }
        Solution::new("The solution is", String::from_utf8(data).unwrap())
    }
}

impl D11 {
    fn wrong(pass: &[u8]) -> bool {
        let mut auxilary = pass[0];
        let mut prev = pass[1];
        let mut ok = false;
        for &c in &pass[2..] {
            if c.wrapping_sub(prev) == 1 && c.wrapping_sub(auxilary) == 2 {
                ok = true;
                break;
            }
            auxilary = prev;
            prev = c;
        }
        if !ok {
            return true;
        }
        prev = pass[0];
        auxilary = 0;
        let mut iter = pass.windows(2);
        while let Some(pair) = iter.next() {
            let (&a, &b) = unsafe { (pair.get_unchecked(0), pair.get_unchecked(1)) };
            if a == b && a != prev {
                auxilary += 1;
                prev = a;
            }
        }
        auxilary < 2
    }
    fn increment(pass: &mut [u8]) {
        for c in pass.iter_mut().rev() {
            *c += 1;
            *c += BAD.contains(c) as u8;
            if *c / 26 != 1 {
                break;
            }
            *c %= 26;
        }
    }
}

const BAD: [u8; 3] = [0xA, 0xD, 0xF];

#[cfg(test)]
mod tests {
    use super::Solver;
    #[test]
    fn test_increment() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 25, 25];
        super::D11::increment(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 7, 0, 0]);
        super::D11::increment(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 7, 0, 1]);
    }

    #[test]
    fn test_part_one() {
        let data = b"abchzxkj".to_vec();
        let solver = super::D11;
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "abchzzmm");
        let data = b"abcdefgh".to_vec();
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "abcdffaa");
    }
}
