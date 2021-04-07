use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct DollarValue {
    pub value: f64,
}

impl DollarValue {
    fn new(value: f64) -> Self {
        DollarValue { value }
    }
}

impl Display for DollarValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub period_start: DateTime<Utc>,
    pub period_length: usize,
    pub symbol: String,
    pub price: Option<DollarValue>,
    pub change_percentage: Option<String>,
    pub min: Option<DollarValue>,
    pub max: Option<DollarValue>,
    pub avg: Option<DollarValue>,
}

impl Report {
    pub fn new(period_start: DateTime<Utc>, symbol: String, adjclose_series: &[f64]) -> Self {
        let current_datetime = Utc::now();
        let period_length = current_datetime
            .signed_duration_since(period_start)
            .num_days() as usize;
        let avg = n_window_sma(period_length, adjclose_series)
            .map(|avgs| *avgs.last().unwrap())
            .map(DollarValue::new);
        let min = min(adjclose_series).map(DollarValue::new);
        let max = max(adjclose_series).map(DollarValue::new);

        let (change_percentage, price) = match price_diff(adjclose_series) {
            Some((perc, absolute_diff)) => (
                Some(format!("{:.2}%", perc)),
                Some(DollarValue::new(absolute_diff)),
            ),
            None => (None, None),
        };
        Report {
            period_start,
            symbol,
            period_length,
            min,
            max,
            avg,
            change_percentage,
            price,
        }
    }
}

pub fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None;
    }
    let mut min = series[0];
    for v in series {
        if *v < min {
            min = *v
        }
    }
    Some(min)
}

pub fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        return None;
    }
    let mut max = 0_f64;
    for v in series {
        if *v > max {
            max = *v
        }
    }
    Some(max)
}

pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.is_empty() {
        return None;
    }

    let mut data = series.iter().peekable();

    let mut windows: Vec<Vec<f64>> = vec![];
    while data.peek().is_some() {
        let window: Vec<f64> = data.by_ref().take(n).cloned().collect();
        windows.push(window)
    }
    let window_sma: Vec<f64> = windows
        .iter()
        .map(|window| {
            let window_sum: f64 = window.iter().by_ref().sum();
            let period = window.iter().by_ref().len() as f64;
            window_sum / period
        })
        .collect();
    Some(window_sma)
}

pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if series.len() <= 1 {
        return None;
    }
    let first_element = *series.first().unwrap();
    let last_element = *series.last().unwrap();

    let percentage = (last_element / first_element) * 100f64;
    let absolute_diff = (last_element - first_element).abs();
    Some((percentage, absolute_diff))
}
