use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, TimeZone, Weekday};
use std::convert::TryInto;
use std::io;
use std::ops::Add;
use sunrise::sunrise_sunset;

fn main() {
    // get current date
    let mut current_date: DateTime<Local> = Local::now();

    // loop over the next 31 days
    let mut counter: i32 = 0;
    while counter < 31 {
        // find the favorable interval number
        let interval_number = get_interval_number(
            current_date
                .weekday()
                .number_from_monday()
                .try_into()
                .unwrap(),
        );
        // get the favorable interval for current day
        let interval_favorabil = get_interval(current_date, interval_number);
        println!(
            "{:8} {}.{:5} interval favorabil: {} - {}",
            translate_weekdays(current_date.weekday()),
            current_date.format("%d"),
            current_date.format("%m,"),
            interval_favorabil.0.format("%H:%M"),
            interval_favorabil.1.format("%H:%M")
        );

        current_date = current_date.add(Duration::days(1));
        counter = counter + 1;
    }

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
}

// find the interval in a day given the number
fn get_interval(day: DateTime<Local>, number: i32) -> (NaiveTime, NaiveTime) {
    let (ora_rasarit_timestamp, ora_apus_timestamp) =
        sunrise_sunset(44.43225, 26.10626, day.year(), day.month(), day.day());

    // Create a normal DateTime from the NaiveDateTime
    let rasarit: DateTime<Local> = Local.timestamp(ora_rasarit_timestamp, 0);
    let apus: DateTime<Local> = Local.timestamp(ora_apus_timestamp, 0);

    let interval = (apus.time() - rasarit.time()) / 8;

    let inceput = rasarit.time() + interval * (number - 1);
    let sfarsit = inceput + interval;

    (inceput, sfarsit)
}

// get the favorable interval for day of week
fn get_interval_number(day_of_week: i32) -> i32 {
    let yamarta = 7 - day_of_week;
    if yamarta != 0 {
        yamarta
    } else {
        7
    }
}

// translate weekedays in Romanian
fn translate_weekdays(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Luni",
        Weekday::Tue => "Marți",
        Weekday::Wed => "Miercuri",
        Weekday::Thu => "Joi",
        Weekday::Fri => "Vineri",
        Weekday::Sat => "Sâmbătă",
        Weekday::Sun => "Duminică",
    }
}
