use chrono::{DateTime, NaiveDate, ParseError, Utc};
use std::fmt;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Input<'a> {
    pub tickers: Vec<&'a str>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl<'a> Input<'a> {
    pub fn new(tickers: Vec<&'a str>, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Input {
            tickers,
            start,
            end,
        }
    }

    pub fn try_new(tickers: Vec<&'a str>, start: &str, end: &str) -> Result<Input<'a>, InputError> {
        let start = Input::parse_date(start)
            .map_err(|e| InputError::new(format!("start date parse error:\n{}", e)))?;
        let end = Input::parse_date(end)
            .map_err(|e| InputError::new(format!("end date parse error:\n{}", e)))?;
        Ok(Input::new(tickers, start, end))
    }

    fn parse_date(input: &str) -> Result<DateTime<Utc>, ParseError> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .map(|date| DateTime::<Utc>::from_utc(date.and_hms_milli(0, 0, 0, 0), Utc))
    }
}

#[derive(Debug)]
pub struct InputError {
    message: String,
}

impl InputError {
    pub fn new(message: String) -> Self {
        InputError { message }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InputError {}
