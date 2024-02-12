use chrono::NaiveDate;
use std::fmt::{Debug, Display};

pub trait Calendar: Display {
    fn mark(&mut self, date: NaiveDate);
    fn unmark(&mut self, date: NaiveDate);
    fn is_marked(&self, date: NaiveDate) -> bool;
    fn rows(&self) -> usize;
    fn day_width(&self) -> usize;
}

impl Debug for dyn Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// #[derive(Debug)]
// pub struct YearCalendar {
//     year: i32,
//     calendars: Calendars,
// }
