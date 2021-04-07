# Maha

```
USAGE:
    maha [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --end <END>             The inclusive end date of the expected range of data. (Format: YYYY-MM-DD)
    -s, --start <START>         The inclusive start date of the expected range of data (Format: YYYY-MM-DD)
    -t, --ticker <TICKER>...    The ticker symbol representing a stock
```

### Example

```
maha  -t goog -t ibm -t aapl -s 2021-03-06 -e 2021-04-07
```
will produce
```
period_start,symbol,price,change %,min,max,30d avg
2021-03-08 14:30:00 UTC,goog,$200.58,109.91%,$2024.17,$2225.55,$2077.28
2021-03-08 14:30:00 UTC,ibm,$9.41,107.54%,$124.18,$136.38,$130.70
2021-03-08 14:30:00 UTC,aapl,$9.85,108.47%,$116.36,$126.21,$121.98
```