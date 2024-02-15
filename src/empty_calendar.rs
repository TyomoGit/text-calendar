use std::fmt::Display;

use crate::Calendar;

#[derive(Debug, Default)]
pub struct EmptyCalendar {
    height: usize,
    width: usize,
}

impl EmptyCalendar {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }
}

impl Calendar for EmptyCalendar {
    fn mark(&mut self, _date: chrono::prelude::NaiveDate) {}
    fn unmark(&mut self, _date: chrono::prelude::NaiveDate) {}

    fn is_marked(&self, _date: chrono::prelude::NaiveDate) -> bool {
        false
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl Display for EmptyCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(self.height) {
            write!(f, "{: ^width$}", "", width = self.width)?;

            if i != self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
