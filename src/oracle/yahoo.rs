use super::{Oracle, OracleError, OracleResult, OHLC};
use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};
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
        let res = self.provider.get_quote_history(ticker, start, end);
        match res {
            Ok(data) => self.quotes(data),
            Err(e) => Err(OracleError {
                message: format!("{}", e),
            }),
        }
    }
}

impl Yahoo {
    fn quotes(&self, resp: YResponse) -> OracleResult<Vec<OHLC>> {
        let maybe_quotes = resp.quotes();
        match maybe_quotes {
            Ok(quotes) => {
                let ohlc_data: Vec<OHLC> = quotes
                    .iter()
                    .map(|quote| self.quote_to_ohlc(quote))
                    .collect();
                Ok(ohlc_data)
            }
            Err(e) => Err(OracleError {
                message: format!("{}", e),
            }),
        }
    }

    fn quote_to_ohlc(&self, q: &Quote) -> OHLC {
        let utc_datetime: DateTime<Utc> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(q.timestamp));
        OHLC {
            open: q.open,
            high: q.high,
            low: q.low,
            adjclose: q.adjclose,
            utc_datetime,
        }
    }
}
