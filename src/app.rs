use std::fmt;
use std::{error::Error, fmt::Display};

use chrono::{DateTime, NaiveDate, ParseError, Utc};
use clap::{Arg, ArgMatches};

#[derive(Debug)]
pub struct Input<'a> {
    pub ticker: &'a str,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug)]
pub struct InputError {
    message: String,
}

impl InputError {
    fn new(message: String) -> Self {
        InputError { message }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InputError {}

impl<'a> Input<'a> {
    fn new(ticker: &'a str, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Input { ticker, start, end }
    }

    fn try_new(ticker: &'a str, start: &str, end: &str) -> Result<Input<'a>, InputError> {
        let start = Input::parse_date(start)
            .map_err(|e| InputError::new(format!("start date parse error:\n{}", e)))?;
        let end = Input::parse_date(end)
            .map_err(|e| InputError::new(format!("end date parse error:\n{}", e)))?;
        Ok(Input { ticker, start, end })
    }

    fn parse_date(input: &str) -> Result<DateTime<Utc>, ParseError> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .map(|date| DateTime::<Utc>::from_utc(date.and_hms_milli(0, 0, 0, 0), Utc))
    }
}
pub struct App<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let matches = clap::App::new("maha")
        .author("Tuhin Nair")
        .arg(
            Arg::with_name("ticker")
                .short("t")
                .long("ticker")
                .value_name("TICKER")
                .help("The ticker symbol representing a stock")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start_date")
                .short("s")
                .long("start")
                .value_name("START")
                .help("The inclusive start date of the expected range of data (Format: YYYY-MM-DD)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end_date")
                .short("e")
                .long("end")
                .value_name("END")
                .help("The inclusive end date of the expected range of data. (Format: YYYY-MM-DD)")
                .takes_value(true),
        )
        .get_matches();
        App { matches }
    }

    pub fn input(&'a self) -> Result<Input, InputError> {
        let ticker = self
            .matches
            .value_of("ticker")
            .ok_or_else(|| InputError::new("missing ticker sybmol".to_string()))?;
        let start = self
            .matches
            .value_of("start_date")
            .ok_or_else(|| InputError::new("missing start date".to_string()))?;
        let end = self
            .matches
            .value_of("end_date")
            .ok_or_else(|| InputError::new("missing end date".to_string()))?;

        Input::try_new(ticker, start, end)
    }

    pub fn print_help<T: Display>(&self, maybe_message: Option<T>) {
        if let Some(msg) = maybe_message {
            println!("{}\n", msg);
        }
        self.matches.usage();
    }
}
