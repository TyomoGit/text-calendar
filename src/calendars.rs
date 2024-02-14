use std::fmt::{write, Display, Write};

use chrono::NaiveDate;

use crate::{Calendar, CalendarCollection, EmptyCalendar};

/// multiple calendars
#[derive(Debug)]
pub struct Calendars {
    calendars: Vec<Box<dyn Calendar>>,
    title: String,
    cols: usize,
}

impl CalendarCollection for Calendars {}

impl Calendars {
    pub fn new(mut calendars: Vec<Box<dyn Calendar>>, title: String, cols: usize) -> Self {
        let padding = cols - (calendars.len() % cols);

        let last_rows = calendars
            .windows(3)
            .step_by(3)
            .last()
            .unwrap()
            .iter()
            .map(|c| c.rows())
            .max()
            .unwrap_or_default();

        let empty_cal_day_width = calendars.last().unwrap().day_width();

        calendars.extend((0..padding).map(|_| {
            Box::new(EmptyCalendar::new(last_rows, empty_cal_day_width)) as Box<dyn Calendar>
        }));

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

    pub fn push(&mut self, calendar: impl Calendar + 'static) {
        self.calendars.push(Box::new(calendar));
    }

    fn rows_list(&self) -> impl Iterator<Item = usize> + '_ {
        self.calendars
            .windows(self.cols)
            .step_by(self.cols)
            .map(|w| w.iter().map(|c| c.rows()).max().unwrap_or_default())
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
        self.calendars.iter().any(|c| c.is_marked(date))
    }

    fn mark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.mark(date));
    }

    fn unmark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.unmark(date));
    }

    fn rows(&self) -> usize {
        self.rows_list().sum()
    }
}

impl Display for Calendars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = self.day_width() / 2;
        let max_line_width = self
            .calendars
            .windows(self.cols)
            .step_by(self.cols)
            .map(|w| w.iter().map(|c| c.day_width() * 7 + padding).sum::<usize>())
            .max()
            .unwrap_or_default()
            - 4;

        writeln!(f, "{:^width$}  ", self.title, width = max_line_width)?;

        let rows_list: Vec<usize> = self.rows_list().collect();

        let windows = self
            .calendars
            .windows(self.cols)
            .step_by(self.cols)
            .enumerate();

        for (line_count, calendars) in windows {
            let lines_list: Vec<Vec<String>> = calendars
                .iter()
                .map(|c| {
                    c.to_string()
                        .split('\n')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect();

            for i in 0..rows_list[line_count] {
                let mut line = String::new();

                for cal in &lines_list {
                    if let Some(cal_line) = cal.get(i) {
                        write!(&mut line, "{}{}", cal_line, " ".repeat(padding))?;
                    } else {
                        write!(&mut line, "{}", " ".repeat(cal[0].len() + padding))?;
                    }
                }

                for _ in 0..padding {
                    line.pop();
                }

                write!(f, "{}", line)?;

                if line_count < self.calendars.len() / self.cols - 1 || i != rows_list[line_count] - 1 {
                    writeln!(f)?;
                }

            }

            // カレンダーの間
            if line_count < self.calendars.len() / self.cols - 1 {
                writeln!(f)?;
            }
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

        let cals = Calendars::new(c2024, "2024".to_string(), 5);
        println!("{}", cals);
    }
}
