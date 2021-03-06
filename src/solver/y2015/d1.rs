use crate::solver::Solver;

pub(crate) struct D1;

impl Solver for D1 {
    fn validate(&self, input: &[u8]) -> Result<(), String> {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;
        const NWLINE: u8 = 0xA;
        for (i, &b) in input.iter().enumerate() {
            if b == NWLINE {
                break;
            }
            if b != LPAREN && b != RPAREN {
                return Err(format!(
                    "INVALID CHAR: '{}'\nAT POSISTION: {}",
                    char::from(b),
                    i
                ));
            }
        }
        Ok(())
    }
    fn solve_part_one(&self, data: Vec<u8>) -> String {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;

        let mut position = 0;
        for &b in data.iter() {
            match b {
                LPAREN => position += 1,
                RPAREN => position -= 1,
                _ => break,
            }
        }
        format!("THE FINAL SANTA'S POSITION IS AT: {}", position)
    }
    fn solve_part_two(&self, data: Vec<u8>) -> String {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;

        let mut position = 0;
        let mut below_ground = 0;
        for (i, &b) in data.iter().enumerate() {
            match b {
                LPAREN => position += 1,
                RPAREN => position -= 1,
                _ => break,
            }
            if position == -1 {
                below_ground = i + 1;
                break;
            }
        }
        format!(
            "THE INSTRUCTION NUMBER TO GET SANTA BELOW GROUND IS:\n{}",
            below_ground
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        use crate::solver::Solver;
        let test_data = String::from("()()()))").into_bytes();
        let solver = super::D1 {};
        let res = solver.solve_part_one(test_data);
        assert_eq!(res, String::from("THE FINAL SANTA'S POSITION IS AT: -2"));
    }
    #[test]
    fn test_part_two() {
        use crate::solver::Solver;
        let test_data = String::from("()()()))").into_bytes();
        let solver = super::D1 {};
        let res = solver.solve_part_two(test_data);
        assert_eq!(
            res,
            String::from("THE INSTRUCTION NUMBER TO GET SANTA BELOW GROUND IS:\n7")
        );
    }
}
