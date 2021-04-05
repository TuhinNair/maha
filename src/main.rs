use chrono::{DateTime, Utc};

mod oracle;
use oracle::{Oracle, OracleResult, Yahoo, OHLC};

mod app;
use app::{App, Input, InputError};

fn main() {
    let app = App::new();
    let input = app.input();

    match input {
        Ok(i) => process_input(i),
        Err(e) => app.print_help(Some(e)),
    };
}

fn process_input(input: Input) {
    let Input { ticker, start, end } = input;
    match fetch(ticker, start, end) {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{}", e),
    };
}

fn fetch(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> OracleResult<Vec<OHLC>> {
    let oracle = Yahoo::new_oracle();
    oracle.ohlc_inclusive_range(ticker, start, end)
}
