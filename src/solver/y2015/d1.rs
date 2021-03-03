use crate::solver::Solver;

pub(crate) struct D1;

impl Solver for D1 {
    fn validate(&self, input: &[u8]) -> bool {
        const LPAREN: u8 = 0x28;
        const RPAREN: u8 = 0x29;
        for &b in input.iter() {
            if b != LPAREN && b != RPAREN {
                return false;
            }
        }
        true
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
