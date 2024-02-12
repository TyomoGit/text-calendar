use std::{collections::HashSet, fmt::Display, ops::RangeInclusive};

use chrono::{Datelike, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;

use crate::{BasicMarker, Calendar, Marker};

#[derive(Debug)]
pub struct MonthCalendar {
    year: i32,
    month: u32,
    // weeks: Vec<NaiveWeek>,
    weeks: Vec<RangeInclusive<u32>>,

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
        marker: impl Marker + 'static,
    ) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, 1)?;

        let end_weekday = begin_weekday.pred();

        let mut weeks: Vec<RangeInclusive<u32>> = vec![];

        let mut start = date;
        let mut end = date;
        while let Some(succ_date) = end.succ_opt() {
            if succ_date.month0() != date.month0() {
                weeks.push((start.day0() + 1)..=(end.day0() + 1));
                break;
            }

            if end.weekday() == end_weekday {
                weeks.push((start.day0() + 1)..=(end.day0() + 1));
                start = succ_date;
                end = start;
                continue;
            }

            end = succ_date;
        }

        Some(Self {
            year,
            month,
            weeks,
            begin_weekday,
            day_width,
            marked: HashSet::new(),
            marker: Box::new(marker),
        })
    }

    pub fn from_ym(year: i32, month: u32) -> Option<Self> {
        Self::new(year, month, Weekday::Sun, 4, BasicMarker::SquareBrackets)
    }
}

impl Calendar for MonthCalendar {
    fn mark(&mut self, date: NaiveDate) {
        let year = if let (true, year) = date.year_ce() {
            year as i32
        } else {
            -(date.year_ce().1 as i32)
        };

        if self.year == year && self.month == date.month0() + 1 {
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

        let first_week = self.weeks.first().unwrap();
        let last_week = self.weeks.last().unwrap();

        for _ in 1..=(7 - first_week.end()) {
            write!(f, "{: ^width$}", "", width = self.day_width)?;
        }

        let last_i = self.weeks.len() - 1;
        for (i, week) in self.weeks[..].iter().cloned().enumerate() {
            for day in week.into_iter() {
                if self.marked.contains(&day) {
                    if self.marked.contains(&day) {
                        write!(
                            f,
                            "{: ^width$}",
                            self.marker.decorate(&format!("{: ^2}", day)),
                            width = self.day_width
                        )?;
                    } else {
                        write!(f, "{: ^width$}", day, width = self.day_width)?;
                    }
                } else {
                    write!(f, "{: ^width$}", day, width = self.day_width)?;
                }
            }

            if i == last_i {
                for _ in 0..(7 - (last_week.end() - last_week.start()) - 1) {
                    write!(f, "{: ^width$}", "", width = self.day_width)?;
                }
            } else {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub fn str_month_calendar(year: i32, month: u32) -> Option<String> {
    Some(MonthCalendar::from_ym(year, month)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const ROWS_LIST_2024: [usize; 12] = [5, 5, 6, 5, 5, 6, 5, 5, 5, 5, 5, 5];

        for i in 0..12 {
            let cal = MonthCalendar::new(2024, i + 1, Weekday::Sun, 2, BasicMarker::SquareBrackets)
                .unwrap();
            assert_eq!(cal.rows() - 2, ROWS_LIST_2024[i as usize], "{:?}", cal);
        }

        let display_test =
            MonthCalendar::new(2024, 6, Weekday::Sun, 4, BasicMarker::SquareBrackets).unwrap();
        println!("{}", display_test);
    }
}
