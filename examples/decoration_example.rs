use std::fmt::Write;

use chrono::{NaiveDate, Weekday};
use text_calendar::{Calendar, Marker, YearCalendar};

#[derive(Clone)]
struct ExclamationMarker {
    day_width: usize,
}

impl Marker for ExclamationMarker {
    fn decorate(&self, day: &str) -> String {
        let count = self.day_width-2;

        let mut result = String::new();

        for i in (0..count/2) {
            write!(&mut result, "!").unwrap();
        }

        write!(&mut result, "{}", day).unwrap();

        for i in (0..count/2) {
            write!(&mut result, "!").unwrap();
        }
        
        if self.day_width % 2 == 1 {
            write!(&mut result, " ").unwrap();
        }
        result
    }
}

fn main() {
    let day_width = 2 + 2*2;
    let sparkles_marker = ExclamationMarker { day_width };
    let mut year_calendar = YearCalendar::new(2024, Weekday::Mon, day_width, sparkles_marker);

    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 3, 20).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 8, 10).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 9, 22).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 12, 21).unwrap());
    year_calendar.mark(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

    println!("{}", year_calendar);
}