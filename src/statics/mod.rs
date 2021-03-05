use crate::challenge::{Day, Year};
use crate::solver::y2015::d1::D1;

lazy_static! {
    pub static ref YEARS: Vec<Year> = vec![
        Year::new(2015, vec![Day::new(1, Box::new(D1))]),
        Year::new(2016, vec![Day::new(1, Box::new(D1))]),
        Year::new(2017, vec![Day::new(1, Box::new(D1))])
    ];
}
