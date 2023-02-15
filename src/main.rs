use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveTime, Weekday};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let start = NaiveDate::from_isoywd_opt(2023, 1, Weekday::Mon)
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_utc();
    let dates: Vec<NaiveDate> = [
        start,
        start + Days::new(1),
        start + Days::new(2),
        start + Days::new(3),
        start + Days::new(4),
        start + Days::new(5),
        start + Days::new(6),
    ]
    .iter()
    .map(DateTime::date_naive)
    .collect();

    let result: Vec<(&NaiveDate, Vec<Response>)> =
        dates.par_iter().map(fetch).filter_map(Result::ok).collect();

    let map = BTreeMap::from_iter(result);

    dbg!(map);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Response {
    day: u8,
}

fn fetch(date: &NaiveDate) -> Result<(&NaiveDate, Vec<Response>), ()> {
    println!("Sleeping...");
    thread::sleep(Duration::from_secs(1));

    let responses = vec![
        Response {
            day: date.day() as u8,
        },
        Response {
            day: date.day() as u8,
        },
    ];

    Ok((date, responses))
}
