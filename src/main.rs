extern crate chrono;
use chrono::{Utc, NaiveDate, NaiveDateTime};

fn main() {
    let start_day : NaiveDateTime = NaiveDate::from_ymd(2017, 09, 21).and_hms(13,37,00);

    let now = Utc::now();
    let timestamp: i64 = now.timestamp();

    println!("This beautiful journey lasts {} seconds", timestamp - start_day.timestamp());
}
