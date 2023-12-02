use std::fmt::{Debug, Display};
use std::{fmt, fs, io};

#[derive(Debug, PartialEq)]
pub enum Answer {
    String(String),
    Number(u64),
    Float(f64),
    Unimplemented,
}

impl Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::String(s) => write!(f, "{s}"),
            Answer::Number(n) => write!(f, "{n}"),
            Answer::Float(n) => write!(f, "{n}"),
            Answer::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

macro_rules! answer_impl {
    ($answer:ident, $answer_type:ty, { $($type:ty),* }) => {
        $(impl From<$type> for Answer {
            fn from(n: $type) -> Self {
                Self::$answer(n as $answer_type)
            }
        })*
    };
}

answer_impl!(
    Number, u64,
    { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
);

answer_impl!(
    Float, f64,
    { f32, f64 }
);

pub trait Solution {
    fn name(&self) -> &'static str;
    fn p1(&self, input: &str) -> Answer;
    fn p2(&self, input: &str) -> Answer;
    fn is_dummy(&self) -> bool {
        false
    }
}

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u32, day: u32) -> io::Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

/// Load the input for the given year and day.
pub fn load_raw(year: u32, day: u32) -> io::Result<String> {
    let file = format!("input/{year}/{:02}.txt", day);
    fs::read_to_string(file)
}

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}

pub struct DummySolution;

impl Solution for DummySolution {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn p1(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn p2(&self, _input: &str) -> Answer {
        unreachable!()
    }

    fn is_dummy(&self) -> bool {
        true
    }
}
