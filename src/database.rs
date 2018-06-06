use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc, TimeZone};
use fitbit::WeightSeries;
use chrono::offset::FixedOffset;

pub struct TimeSeriesData<Tz> where Tz: TimeZone {
    timestamp: DateTime<Tz>,
    value: f64,
}

impl<Tz> TimeSeriesData<Tz> where Tz: TimeZone {
    pub fn from_weight_series(ws: WeightSeries, offset_millis: i32) -> Option<Self> {
        // Fitbit always returns a "local" date based on the user's locale
        let local_naive_dt = match NaiveDate::parse_from_str(&ws.date, "%Y-%m-%d") {
            Ok(date) => date.and_hms(0, 0, 0),
            Err(_) => return None,
        };
        let value: f64 = match ws.value.parse() {
            Ok(value) => value,
            Err(_) => return None,
        };

        let dt = FixedOffset::west(offset_millis / 1000);
        let dt = DateTime::from_utc(local_naive_dt, dt);

        Some(TimeSeriesData{timestamp: dt, value: value })
    }
}


//fn from(val: NaiveDate) -> Self {
//    let dt = Date::<Utc>::from_utc(val, Utc);
//    Key(dt.and_hms(0, 0, 0).to_rfc3339().as_bytes().to_vec())
//}
