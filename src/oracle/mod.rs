use chrono::{DateTime, Utc};
use std::fmt;
use std::{error::Error, fmt::Display};

type OracleResult<T> = Result<T, OracleError>;
#[derive(Debug)]
struct OracleError {
    message: String,
}

impl Display for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for OracleError {}

struct OHLC {
    utc_datetime: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    adjclose: f64,
}
trait Oracle {
    fn new_oracle() -> Self;

    fn ohlc_inclusive_range(
        ticker: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Option<OHLC>>, OracleError>;
}
