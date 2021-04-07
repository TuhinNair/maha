use std::io;
use std::{error::Error, fmt::Display};
use std::{fmt, io::Write};

#[derive(Debug, Clone)]
pub struct CSV {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct CSVErr {
    message: String,
}

impl Display for CSVErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CSVErr {}

impl From<io::Error> for CSVErr {
    fn from(e: io::Error) -> Self {
        CSVErr {
            message: format!("{}", e),
        }
    }
}

impl CSV {
    pub fn new(headers: Vec<String>) -> Self {
        let data = vec![];
        Self { headers, data }
    }

    pub fn add_line(&mut self, line: Vec<String>) -> Result<(), CSVErr> {
        if line.len() != self.headers.len() {
            return Err(CSVErr {
                message: "invalid csv data".to_string(),
            });
        };

        self.data.push(line);
        Ok(())
    }

    pub fn write<T: Write>(&self, buffer: &mut T, delimeter: &str) -> Result<(), CSVErr> {
        let mut headers = self.headers.clone().join(delimeter);
        let mut lines = self
            .data
            .clone()
            .iter()
            .map(|l| l.join(delimeter))
            .collect::<Vec<String>>();
        let mut data = vec![headers];
        data.append(&mut lines);

        let bytes = &data.join("\n").into_bytes()[..];

        buffer.write_all(bytes)?;

        Ok(())
    }
}
