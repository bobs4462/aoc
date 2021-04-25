use crate::challenge::{Day, Year};
use crate::solver::y2015::*;

lazy_static! {
    pub static ref YEARS: Vec<Year> = vec![
        Year::new(
            2015,
            vec![
                Day::new(1, Box::new(D1)),
                Day::new(2, Box::new(D2)),
                Day::new(3, Box::new(D3)),
                Day::new(4, Box::new(D4)),
                Day::new(5, Box::new(D5)),
                Day::new(6, Box::new(D6)),
                Day::new(7, Box::new(D7)),
                Day::new(8, Box::new(D8)),
                Day::new(9, Box::new(D9)),
                Day::new(10, Box::new(D10)),
                Day::new(11, Box::new(D11)),
                Day::new(12, Box::new(D12)),
                Day::new(13, Box::new(D13)),
                Day::new(14, Box::new(D14)),
                Day::new(15, Box::new(D15)),
            ]
        ),
        Year::new(2016, vec![Day::new(1, Box::new(D1))]),
        Year::new(2017, vec![Day::new(1, Box::new(D1))])
    ];
}
