use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use chrono::offset::{TimeZone};
use fitbit::body::WeightSeries;

pub struct TimeSeriesData {
    timestamp: DateTime<Utc>,
    value: f64,
}

impl TimeSeriesData {
}


//fn from(val: NaiveDate) -> Self {
//    let dt = Date::<Utc>::from_utc(val, Utc);
//    Key(dt.and_hms(0, 0, 0).to_rfc3339().as_bytes().to_vec())
//}
