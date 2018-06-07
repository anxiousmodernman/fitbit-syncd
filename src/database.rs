use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use chrono::offset::{TimeZone};
use fitbit::body::WeightSeries;

pub struct TimeSeriesData {
    timestamp: DateTime<Utc>,
    value: f64,
}

impl TimeSeriesData {
}


