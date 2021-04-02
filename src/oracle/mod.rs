use chrono::{DateTime, Utc};
use yahoo_finance_api as yahoo;

struct OHLC {}
trait Oracle {
    fn new_provider() -> Self;

    fn ohlc_inclusive_range(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> OHLC;
}
