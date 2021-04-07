use clap::{Arg, ArgMatches};
pub use input::{Input, InputError};
use std::{env::Args, fmt::Display};

mod input;

pub struct App<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> App<'a> {
    pub fn new(args: Args) -> Self {
        let matches = clap::App::new("maha")
        .author("Tuhin Nair")
        .arg(
            Arg::with_name("ticker")
                .short("t")
                .long("ticker")
                .value_name("TICKER")
                .help("The ticker symbol representing a stock")
                .multiple(true)
                .number_of_values(1)
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
        .get_matches_from(args);
        App { matches }
    }

    pub fn parse_input(&'a self) -> Result<Input, InputError> {
        let tickers = self
            .matches
            .values_of("ticker")
            .map(|vals| vals.collect::<Vec<&str>>())
            .ok_or_else(|| InputError::new("missing ticker sybmol".to_string()))?;
        let start = self
            .matches
            .value_of("start_date")
            .ok_or_else(|| InputError::new("missing start date".to_string()))?;
        let end = self
            .matches
            .value_of("end_date")
            .ok_or_else(|| InputError::new("missing end date".to_string()))?;

        Input::try_new(tickers, start, end)
    }

    pub fn print_help<T: Display>(&self, maybe_message: Option<T>) {
        if let Some(msg) = maybe_message {
            println!("{}\n", msg);
        }
        println!("{}", self.matches.usage());
    }
}
