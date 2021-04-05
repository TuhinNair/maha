use chrono::{DateTime, Utc};
use std::fmt;
use std::{error::Error, fmt::Display};

mod yahoo;
pub use yahoo::{Yahoo, YahooError};

pub type OracleResult<T> = Result<T, OracleError>;
#[derive(Debug)]
pub enum OracleError {
    Yahoo(YahooError),
}

impl Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OracleError::Yahoo(er) => write!(f, "Yahoo Error: {}", er),
        }
    }
}

impl From<YahooError> for OracleError {
    fn from(err: YahooError) -> Self {
        OracleError::Yahoo(err)
    }
}

impl Error for OracleError {}

#[derive(Debug)]
pub struct OHLC {
    utc_datetime: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    adjclose: f64,
}
pub trait Oracle {
    fn new_oracle() -> Self;

    fn ohlc_inclusive_range(
        &self,
        ticker: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> OracleResult<Vec<OHLC>>;
}
