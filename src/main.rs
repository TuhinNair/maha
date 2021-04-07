use std::io::Write;

use app::{App, Input};
use chrono::{DateTime, Utc};
use csv::{CSVErr, CSV};
use oracle::{Oracle, OracleResult, Yahoo, OHLC};
use stats::Report;

mod app;
mod csv;
mod oracle;
mod stats;

fn main() {
    let app = App::new(std::env::args());
    let input = app.parse_input();

    match input {
        Ok(i) => process_input(i),
        Err(e) => app.print_help(Some(e)),
    };
}

fn process_input(input: Input) {
    let Input { ticker, start, end } = input;

    match fetch(ticker, start, end) {
        Ok(data) => {
            let report = make_report(data);
            let reports = vec![report];
            let mut buffer = std::io::stdout();
            if let Err(e) = print_csv(&mut buffer, reports) {
                println!("csv error: {}", e);
            }
        }
        Err(e) => println!("{}", e),
    };
}

fn fetch(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> OracleResult<Vec<OHLC>> {
    let oracle = Yahoo::new_oracle();
    oracle.ohlc_inclusive_range(ticker, start, end)
}

fn make_report(data: Vec<OHLC>) -> Report {
    let datum = data.first().unwrap();
    let period_start = datum.utc_datetime;
    let symbol = datum.symbol.clone();
    let adjclose_series = data.iter().map(|d| d.adjclose).collect::<Vec<f64>>();
    Report::new(period_start, symbol, &adjclose_series[..])
}

fn print_csv<T: Write>(buffer: &mut T, reports: Vec<Report>) -> Result<(), CSVErr> {
    let headers: Vec<String> = vec![
        "period_start".to_owned(),
        "symbol".to_owned(),
        "price".to_owned(),
        "change %".to_owned(),
        "min".to_owned(),
        "max".to_owned(),
        "30d avg".to_owned(),
    ];
    let mut csv_report = CSV::new(headers);
    let report_lines = reports
        .iter()
        .map(|report| {
            vec![
                report.period_start.to_string(),
                report.symbol.clone(),
                report
                    .price
                    .map(|dv| dv.to_string())
                    .unwrap_or("$0.0".to_string()),
                report
                    .change_percentage
                    .clone()
                    .unwrap_or("0.0".to_string()),
                report
                    .min
                    .map(|dv| dv.to_string())
                    .unwrap_or("$0.0".to_string()),
                report
                    .max
                    .map(|dv| dv.to_string())
                    .unwrap_or("$0.0".to_string()),
                report
                    .avg
                    .map(|dv| dv.to_string())
                    .unwrap_or("$0.0".to_string()),
            ]
        })
        .collect::<Vec<Vec<String>>>();
    for report_line in report_lines {
        csv_report.add_line(report_line)?;
    }
    csv_report.write(buffer, ",")
}
