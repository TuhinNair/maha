use chrono::{DateTime, Utc};
use std::fmt;
use std::{error::Error, fmt::Display};
pub use yahoo::{Yahoo, YahooError};

mod yahoo;

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

impl Error for OracleError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            OracleError::Yahoo(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct OHLC {
    pub symbol: String,
    pub utc_datetime: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub adjclose: f64,
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
