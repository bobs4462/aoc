use crate::challenge::{Day, Year};
use crate::solver::y2015::{d1::D1, d2::D2, d3::D3};

lazy_static! {
    pub static ref YEARS: Vec<Year> = vec![
        Year::new(
            2015,
            vec![
                Day::new(1, Box::new(D1)),
                Day::new(2, Box::new(D2)),
                Day::new(3, Box::new(D3))
            ]
        ),
        Year::new(2016, vec![Day::new(1, Box::new(D1))]),
        Year::new(2017, vec![Day::new(1, Box::new(D1))])
    ];
}
