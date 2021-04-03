use chrono::{DateTime, Utc};
use std::fmt;
use std::{error::Error, fmt::Display};

mod yahoo;
pub use yahoo::Yahoo;

pub type OracleResult<T> = Result<T, OracleError>;
#[derive(Debug)]
pub struct OracleError {
    pub message: String,
}

impl Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
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
