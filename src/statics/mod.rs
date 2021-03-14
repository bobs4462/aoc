use crate::challenge::{Day, Year};
use crate::solver::y2015::*;

lazy_static! {
    pub static ref YEARS: Vec<Year> = vec![
        Year::new(
            2015,
            vec![
                Day::new(1, Box::new(d1::D1)),
                Day::new(2, Box::new(d2::D2)),
                Day::new(3, Box::new(d3::D3)),
                Day::new(4, Box::new(d4::D4))
            ]
        ),
        Year::new(2016, vec![Day::new(1, Box::new(d1::D1))]),
        Year::new(2017, vec![Day::new(1, Box::new(d1::D1))])
    ];
}
