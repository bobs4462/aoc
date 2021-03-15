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
        let mut nice = 0;
        let prohibited = ["ab", "cd", "pq", "xy"];
        let vowels = b"aeiou";
        'words: for l in lines {
            let word = unsafe { std::str::from_utf8_unchecked(l) };
            for &pr in &prohibited {
                if let Some(_) = word.find(pr) {
                    continue 'words;
                }
            }
            let mut prev: u8 = 0;
            let mut is_nice = false;
            for &c in l {
                if c == prev {
                    is_nice = true;
                    break;
                }
                prev = c;
            }
            if !is_nice {
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
                nice += 1;
            }
        }
        Solution::new("Nice words count is:", nice.to_string())
    }
    fn solve_part_two(&self, data: Vec<u8>) -> Solution {
        let lines = data.split(|c| c.is_ascii_whitespace());
        let mut nice = 0;
        for l in lines {
            let mut pairs = Vec::new();
            let mut prev: &[u8] = b"";
            let len = l.len() - 1;
            for i in 0..len {
                let cur = &l[i..(i + 2)];
                if prev == cur {
                    // println!("{}", std::str::from_utf8(l).unwrap());
                    prev = b"";
                    continue;
                }
                pairs.push(cur);
                prev = cur;
            }
            let before = pairs.len();
            pairs.sort();
            pairs.dedup();
            let after = pairs.len();
            if before == after {
                continue;
            }
            let mut is_nice = false;
            for i in 2..=len {
                if l[i - 2] == l[i] {
                    is_nice = true;
                    break;
                }
            }
            if is_nice {
                nice += 1;
            }
        }
        Solution::new("Nice words count is:", nice.to_string())
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
    #[test]
    fn test_part_two() {
        let data = b"qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy\ndodjadoqyxsuazxt\njjwkrlquazzjbvlm"
            .to_vec();
        let solver = super::D5 {};
        let res = solver.solve_part_two(data);
        assert_eq!(res.value, "3");
    }
}
