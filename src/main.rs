use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveTime, Weekday};
use futures::future::join_all;
use std::collections::BTreeMap;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let start = NaiveDate::from_isoywd_opt(2023, 1, Weekday::Mon)
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_utc();
    let dates: Vec<NaiveDate> = vec![
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

    let futures = dates.iter().map(fetch);

    let result: Vec<(&NaiveDate, Vec<Response>)> = join_all(futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let map = BTreeMap::from_iter(result);

    dbg!(map);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Response {
    day: u8,
}

async fn fetch(date: &NaiveDate) -> Result<(&NaiveDate, Vec<Response>), ()> {
    println!("Sleeping...");
    tokio::time::sleep(Duration::from_secs(1)).await;

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
