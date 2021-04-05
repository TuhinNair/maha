use clap::value_t;
#[macro_use]
use clap::{App, Arg, SubCommand, ArgMatches};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

mod oracle;
use oracle::{Oracle, OracleResult, Yahoo, OHLC};

fn app() -> ArgMatches<'static> {
    App::new("maha")
        .author("Tuhin Nair")
        .arg(
            Arg::with_name("ticker")
                .short("t")
                .long("ticker")
                .value_name("TICKER")
                .help("The ticker symbol representing a stock")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start_date")
                .short("s")
                .long("start")
                .value_name("START")
                .help("The inclusive start date of the expected range of data (Format: YYYY-MM-DD)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end_date")
                .short("e")
                .long("end")
                .value_name("END")
                .help("The inclusive end date of the expected range of data. (Format: YYYY-MM-DD)")
                .takes_value(true),
        )
        .get_matches()
}

fn main() {
    let matches = app();

    let ticker: &str;
    if let Some(t) = matches.value_of("ticker") {
        ticker = t;
    } else {
        println!("{}", matches.usage());
        return;
    }

    let start: DateTime<Utc>;
    if let Some(s) = matches.value_of("start_date") {
        match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(sd) => {
                start = DateTime::<Utc>::from_utc(sd.and_hms_milli(0, 0, 0, 0), Utc);
            }
            Err(e) => {
                println!("{}\n{}", e, matches.usage());
                return;
            }
        }
    } else {
        println!("{}", matches.usage());
        return;
    }

    let end: DateTime<Utc>;
    if let Some(e) = matches.value_of("end_date") {
        match NaiveDate::parse_from_str(e, "%Y-%m-%d") {
            Ok(ed) => {
                end = DateTime::<Utc>::from_utc(ed.and_hms_milli(0, 0, 0, 0), Utc);
            }
            Err(e) => {
                println!("{}\n{}", e, matches.usage());
                return;
            }
        }
    } else {
        println!("{}", matches.usage());
        return;
    }

    match fetch(ticker, start, end) {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{}", e),
    };
}

fn fetch(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> OracleResult<Vec<OHLC>> {
    let oracle = Yahoo::new_oracle();
    oracle.ohlc_inclusive_range(ticker, start, end)
}
