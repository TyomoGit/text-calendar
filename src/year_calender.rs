use std::fmt::Display;

use chrono::Weekday;

use crate::{Calendar, CalendarCollection, Calendars, Marker, MonthCalendar};

pub struct YearCalendar {
    year: i32,
    calendars: Calendars,
}

impl CalendarCollection for YearCalendar {}

impl YearCalendar {
    pub fn new<T: Marker + Clone + 'static>(
        year: i32,
        begin_weekday: Weekday,
        day_width: usize,
        marker: T,
    ) -> Self {
        // let mut calendars = Calendars::empty(year.to_string(), 3);

        let mut calendar_list: Vec<Box<dyn Calendar>> = vec![];

        for month in 1..13 {
            calendar_list.push(Box::new(
                MonthCalendar::new(year, month, begin_weekday, day_width, marker.clone()).unwrap(),
            ));
        }

        let calendars = Calendars::new(calendar_list, year.to_string(), 3);

        Self { year, calendars }
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}

impl Display for YearCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.calendars)
    }
}

impl Calendar for YearCalendar {
    fn is_marked(&self, date: chrono::prelude::NaiveDate) -> bool {
        self.calendars.is_marked(date)
    }

    fn mark(&mut self, date: chrono::prelude::NaiveDate) {
        self.calendars.mark(date)
    }

    fn unmark(&mut self, date: chrono::prelude::NaiveDate) {
        self.calendars.unmark(date)
    }

    fn height(&self) -> usize {
        self.calendars.height()
    }

    fn width(&self) -> usize {
        self.calendars.width()
    }
}
