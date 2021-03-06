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
                return Err(format!("INVALID CHAR: {}\nAT POSISTION: {}", b, i));
            }
        }
        Ok(())
    }
    fn solve_part_one(&self, data: Vec<u8>) -> String {
        drop(data);
        String::from("PART 2")
    }
    fn solve_part_two(&self, data: Vec<u8>) -> String {
        drop(data);
        String::from("PART 2")
    }
}
