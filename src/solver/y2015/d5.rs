/// --- Day 5: Doesn't He Have Intern-Elves For This? ---
pub struct D5;
use crate::solver::{Solution, Solver};

impl Solver for D5 {
    /// Just validate that the input is alphanumeric in ASCII
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        for (i, &c) in input.iter().enumerate() {
            if !c.is_ascii_alphanumeric() && !c.is_ascii_whitespace() {
                return Err(format!("The character at postion {} is invalid: {}", i, c,));
            }
        }
        Ok(())
    }

    fn solve_part_one(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|c| c.is_ascii_whitespace());
        let mut nice = Vec::new();
        let prohibited = ["ab", "cd", "pq", "xy"];
        let vowels = b"aeiou";
        for l in lines {
            let word = unsafe { std::str::from_utf8_unchecked(l) };
            let mut is_naughty = false;
            for &pr in &prohibited {
                if let Some(_) = word.find(pr) {
                    is_naughty = true;
                    break;
                }
            }
            if is_naughty {
                continue;
            }
            let mut prev: u8 = 0;
            is_naughty = true;
            for &c in l {
                if c == prev {
                    is_naughty = false;
                    break;
                }
                prev = c;
            }
            if is_naughty {
                continue;
            }
            let mut vowel_count = 0;
            for v in vowels {
                for c in l {
                    if c == v {
                        vowel_count += 1;
                    }
                }
            }
            if vowel_count > 2 {
                nice.push(l);
            }
        }
        Solution::new("Nice words count is:", nice.len().to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        drop(data);
        Solution::new("", String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    #[test]
    fn test_part_one() {
        let data =
            b"ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb".to_vec();
        let solver = super::D5 {};
        let res = solver.solve_part_one(data);
        assert_eq!(res.value, "2");
    }
}
