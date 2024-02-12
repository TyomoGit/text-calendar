use std::fmt::Display;

use crate::Calendar;

pub struct EmptyCalendar {
    rows: usize,
    day_width: usize,
}

impl EmptyCalendar {
    pub fn new(rows: usize, day_width: usize) -> Self {
        Self { rows, day_width }
    }
}

impl Calendar for EmptyCalendar {
    fn day_width(&self) -> usize {
        self.day_width
    }

    fn mark(&mut self, _date: chrono::prelude::NaiveDate) {}
    fn unmark(&mut self, _date: chrono::prelude::NaiveDate) {}

    fn is_marked(&self, _date: chrono::prelude::NaiveDate) -> bool {
        false
    }

    fn rows(&self) -> usize {
        self.rows
    }
}

impl Display for EmptyCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..(self.rows) {
            writeln!(f, "{: ^width$}", "", width = self.day_width * 7)?;
        }

        Ok(())
    }
}
