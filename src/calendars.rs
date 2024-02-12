use std::fmt::{Display, Write};

use chrono::NaiveDate;

use crate::{Calendar, CalendarCollection};

/// multiple calendars
#[derive(Debug)]
pub struct Calendars {
    calendars: Vec<Box<dyn Calendar>>,
    title: String,
    cols: usize,
}

impl CalendarCollection for Calendars {}

impl Calendars {
    pub fn new(calendars: Vec<Box<dyn Calendar>>, title: String, cols: usize) -> Self {
        Self {
            calendars,
            title,
            cols,
        }
    }

    pub fn empty(title: String, cols: usize) -> Self {
        Self {
            calendars: vec![],
            title,
            cols,
        }
    }

    pub fn push(&mut self, calendar: Box<dyn Calendar>) {
        self.calendars.push(calendar);
    }

    fn rows_list(&self) -> Vec<usize> {
        self
            .calendars
            .windows(3)
            .map(|w| w.iter().map(|c| c.rows()).max().unwrap_or_default())
            .collect()
    }
}

impl Calendar for Calendars {
    fn day_width(&self) -> usize {
        self.calendars
            .iter()
            .map(|c| c.day_width())
            .max()
            .unwrap_or_default()
    }

    fn is_marked(&self, date: NaiveDate) -> bool {
        self.calendars
            .iter()
            .any(|c| c.is_marked(date))
    }

    fn mark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.mark(date));
    }

    fn unmark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.unmark(date));
    }

    fn rows(&self) -> usize {
        self.rows_list().iter().sum()
    }
}

impl Display for Calendars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = 2;
        let max_line_width = self.calendars.windows(3)
            .map(|w| {
                w.iter()
                    .map(|c| c.day_width()*7+padding)
                    .sum::<usize>()
            })
            .max()
            .unwrap_or_default() - 4;

        writeln!(f, "{:^width$}  ", self.title, width = max_line_width)?;

        let rows_list = self.rows_list();

        for (row, calendars) in self
            .calendars
            .windows(self.cols)
            .step_by(self.cols)
            .enumerate()
        {
            let lines_list: Vec<Vec<String>> = calendars
                .iter()
                .map(|c| {
                    c.to_string()
                        .split('\n')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect();

            for i in 0..rows_list[row] {
                let mut line = String::new();

                for cal in &lines_list {
                    write!(&mut line, "{}{}", cal[i], " ".repeat(padding))?;
                }

                for _ in 0..padding { line.pop(); }

                writeln!(f, "{}", line)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{n_date, MonthCalendar};

    use super::*;

    #[test]
    fn test() {
        let mut c2024: Vec<Box<dyn Calendar>> = vec![];

        for month in 1..13 {
            c2024.push(Box::new(MonthCalendar::from_ym(2024, month).unwrap()));
        }

        c2024[1 - 1].mark(n_date!(2024, 1, 1));
        c2024[2 - 1].mark(n_date!(2024, 2, 10));
        c2024[2 - 1].mark(n_date!(2024, 2, 13));
        c2024[6 - 1].mark(n_date!(2024, 6, 27));
        c2024[8 - 1].mark(n_date!(2024, 8, 10));
        c2024[8 - 1].mark(n_date!(2024, 8, 11));

        let cals = Calendars::new(c2024, "2024".to_string(), 3);
        println!("{}", cals);
    }
}
