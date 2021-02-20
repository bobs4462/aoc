use std::fmt::{Display, Formatter};

pub enum Year {
    Y0000,
    Y2015,
    Y2016,
    Y2017,
    Y2018,
    Y2019,
    Y2020,
}

pub struct Config {
    year: Year,
    challenge: usize,
    part: usize,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_repr = match self {
            Year::Y0000 => "Year not set",
            Year::Y2015 => "Year 2015",
            Year::Y2016 => "Year 2016",
            Year::Y2017 => "Year 2017",
            Year::Y2018 => "Year 2018",
            Year::Y2019 => "Year 2019",
            Year::Y2020 => "Year 2020",
        };
        write!(f, "{}", str_repr)
    }
}

impl Year {
    pub fn vec() -> Vec<Year> {
        let variants: Vec<Year> = vec![
            Year::Y2015,
            Year::Y2016,
            Year::Y2017,
            Year::Y2018,
            Year::Y2019,
            Year::Y2020,
        ];
        variants
    }
}

impl Config {
    pub fn reset(&mut self) {
        self.year = Year::Y0000;
        self.challenge = 0;
        self.part = 0;
    }

    pub fn with_values(year: Year, challenge: usize, part: usize) -> Self {
        Config {
            year,
            challenge,
            part,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            year: Year::Y0000,
            challenge: 0,
            part: 0,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{year}. Challenge {challenge}, part {part}",
            year = self.year,
            challenge = self.challenge,
            part = self.part,
        )
    }
}
