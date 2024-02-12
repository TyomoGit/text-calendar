use std::{collections::HashSet, fmt::Display};

use chrono::{Datelike, Month, NaiveDate, NaiveWeek, Weekday};
use num_traits::FromPrimitive;

use crate::{BasicMarker, Calendar, Marker};

#[derive(Debug)]
pub struct MonthCalendar {
    year: i32,
    month: u32,
    weeks: Vec<NaiveWeek>,

    begin_weekday: Weekday,
    day_width: usize,

    marker: Box<dyn Marker>,
    marked: HashSet<u32>,
}

impl MonthCalendar {
    pub fn new(
        year: i32,
        month: u32,
        begin_weekday: Weekday,
        day_width: usize,
        marker: Box<dyn Marker>,
    ) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, 1)?;

        let mut weeks = vec![];

        for week in date.iter_weeks().take(6) {
            if week.month() > month {
                break;
            }

            weeks.push(week.week(begin_weekday));
        }

        Some(Self {
            year,
            month,
            weeks,
            begin_weekday,
            day_width,
            marked: HashSet::new(),
            marker,
        })
    }

    pub fn from_ym(year: i32, month: u32) -> Option<Self> {
        Self::new(
            year,
            month,
            Weekday::Sun,
            4,
            Box::new(BasicMarker::SquareBrackets),
        )
    }
}

impl Calendar for MonthCalendar {
    fn mark(&mut self, date: NaiveDate) {
        let year = if let (true, year) = date.year_ce(){
            year as i32
        } else  {
            -(date.year_ce().1 as i32)
        };


        if self.year == year && self.month == date.month0()+1 {
            self.marked.insert(date.day());
        }
    }

    fn unmark(&mut self, date: NaiveDate) {
        self.marked.remove(&date.day());
    }

    fn is_marked(&self, date: NaiveDate) -> bool {
        self.marked.contains(&date.day())
    }

    fn rows(&self) -> usize {
        self.weeks.len() + 2
    }

    fn day_width(&self) -> usize {
        self.day_width
    }
}

impl Display for MonthCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{: ^width$}",
            Month::from_u32(self.month).unwrap().name(),
            width = self.day_width * 7
        )?;

        let mut weekday = self.begin_weekday;
        let weekday_width = if self.day_width > 4 { 3 } else { 2 };
        for _ in 0..7 {
            write!(
                f,
                "{: ^width$}",
                &weekday.to_string()[..weekday_width],
                width = self.day_width
            )?;
            weekday = weekday.succ();
        }

        writeln!(f)?;

        for week in &self.weeks {
            let mut day = week.first_day();
            for _ in 0..7 {
                if day.month() == self.month {
                    if self.marked.contains(&day.day()) {
                        write!(
                            f,
                            "{: ^width$}",
                            self.marker.decorate(&format!("{: ^2}", day.day())),
                            width = self.day_width
                        )?;
                    } else {
                        write!(f, "{: ^width$}", day.day(), width = self.day_width)?;
                    }
                } else {
                    write!(f, "{: ^width$}", "", width = self.day_width)?;
                }

                day = day.succ_opt().unwrap_or(day);
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn str_month_calendar(year: i32, month: u32) -> Option<String> {
    Some(MonthCalendar::from_ym(year, month)?.to_string())
}

#[cfg(test)]
mod tests {
    use crate::n_date;

    use super::*;

    #[test]
    fn test() {
        let mut c = MonthCalendar::new(
            2024,
            1,
            Weekday::Sun,
            4,
            Box::new(BasicMarker::SquareBrackets),
        )
        .unwrap();
        c.mark(n_date!(2024, 1, 1));
        c.mark(n_date!(2024, 1, 21));
        c.mark(n_date!(2024, 1, 26));
        println!("{}", c);
    }
}
