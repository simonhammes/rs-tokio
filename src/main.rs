use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveTime, Utc, Weekday};
use futures::future::join_all;
use std::collections::BTreeMap;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let start = DateTime::<Utc>::from_utc(
        NaiveDate::from_isoywd_opt(2023, 1, Weekday::Mon)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        Utc,
    );
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

    let mut result = BTreeMap::new();
    for date in &dates {
        result.insert(*date, vec![]);
    }

    let futures = result
        .iter_mut()
        .map(|(date, responses)| fetch(responses, *date));

    join_all(futures).await;

    dbg!(result);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Response {
    day: u8,
}

async fn fetch(result: &mut Vec<Response>, date: NaiveDate) -> Result<(), ()> {
    println!("Sleeping...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    *result = vec![
        Response {
            day: date.day() as u8,
        },
        Response {
            day: date.day() as u8,
        },
    ];

    Ok(())
}
