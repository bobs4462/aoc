use std::{
    fmt::{Display, Formatter},
    time::Duration,
};
use termion::event::Key;

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
    pub year: Year,
    pub challenge: usize,
    pub part: usize,
    pub quit_key: Key,
    pub tick_rate: Duration,
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

    pub fn with_values(
        year: Year,
        challenge: usize,
        part: usize,
        quit_key: char,
        tick_rate: u64,
    ) -> Self {
        Config {
            year,
            challenge,
            part,
            quit_key: Key::Char(quit_key),
            tick_rate: Duration::from_millis(tick_rate),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            year: Year::Y0000,
            challenge: 0,
            part: 0,
            quit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(100),
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
