use chrono::{DateTime, TimeZone, Utc};

mod oracle;
use oracle::{Oracle, Yahoo};

fn main() {
    let oracle = Yahoo::new_oracle();
    let start = Utc.ymd(2021, 03, 03).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2021, 03, 05).and_hms_milli(0, 0, 0, 0);
    let res = oracle.ohlc_inclusive_range("IBM", start, end);
    match res {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{}", e.message),
    };
}
