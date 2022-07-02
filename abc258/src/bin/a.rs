use proconio::input;
use chrono::{DateTime, Duration, Utc, Local, offset::{FixedOffset, TimeZone}, NaiveDateTime};


fn main() {
    input! {
        k: i64
    }

    let dt: DateTime<Local> = Local.datetime_from_str("2018/12/07 21:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
    let dt1 = dt + Duration::minutes(k);
    let dt2 = dt1.format("%H:%M");
    println!("{}", dt2);
}
