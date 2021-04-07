use super::{Oracle, OracleError, OracleResult, OHLC};
use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};
pub use yahoo_finance_api::YahooError;
use yahoo_finance_api::{Quote, YResponse, YahooConnector};

pub struct Yahoo {
    provider: YahooConnector,
}

impl Oracle for Yahoo {
    fn new_oracle() -> Self {
        let provider = YahooConnector::new();
        Yahoo { provider }
    }

    fn ohlc_inclusive_range(
        &self,
        ticker: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> OracleResult<Vec<OHLC>> {
        let raw_response = self.provider.get_quote_history(ticker, start, end)?;
        self.raw_response_to_ohlc(ticker, raw_response)
    }
}

impl Yahoo {
    fn raw_response_to_ohlc(
        &self,
        symbol: &str,
        raw_response: YResponse,
    ) -> OracleResult<Vec<OHLC>> {
        let ohlc_data = raw_response.quotes().map(|q| {
            q.iter()
                .map(|quote| self.quote_to_ohlc(symbol, quote))
                .collect::<Vec<OHLC>>()
        })?;
        Ok(ohlc_data)
    }

    fn quote_to_ohlc(&self, symbol: &str, q: &Quote) -> OHLC {
        let utc_datetime: DateTime<Utc> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(q.timestamp));
        OHLC {
            open: q.open,
            high: q.high,
            low: q.low,
            adjclose: q.adjclose,
            utc_datetime,
            symbol: symbol.to_owned(),
        }
    }
}
