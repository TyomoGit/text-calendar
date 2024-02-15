use std::fmt::{Display, Write};

use chrono::NaiveDate;

use crate::{Calendar, CalendarCollection, EmptyCalendar};

/// multiple calendars
#[derive(Debug)]
pub struct Calendars {
    calendars: Vec<Box<dyn Calendar>>,
    title: String,
    cols: usize,
    width: usize,
    padding: usize,
}

impl CalendarCollection for Calendars {}

impl Calendars {
    pub fn new(mut calendars: Vec<Box<dyn Calendar>>, title: String, cols: usize) -> Self {
        let last_padding = cols - (calendars.len() % cols);

        let last_height = calendars[calendars.len() - (cols - last_padding)..]
            .iter()
            .map(|c| c.height())
            .max()
            .unwrap_or_default();

        if last_padding != cols {
            let empty_cal_width = calendars.last().unwrap().width();

            calendars.extend((0..last_padding).map(|_| {
                Box::new(EmptyCalendar::new(last_height, empty_cal_width)) as Box<dyn Calendar>
            }));
        }

        let no_padding_width = calendars
            .windows(cols)
            .step_by(cols)
            .map(|w| w.iter().map(|c| c.width()).sum::<usize>())
            .max()
            .unwrap_or_default();

        let padding = no_padding_width / cols / 7;

        let width = no_padding_width + padding * (cols - 1);

        Self {
            calendars,
            title,
            cols,
            width,
            padding,
        }
    }

    fn height_list(&self) -> impl Iterator<Item = usize> + '_ {
        self.calendars
            .windows(self.cols)
            .step_by(self.cols)
            .map(|w| w.iter().map(|c| c.height()).max().unwrap_or_default())
    }
}

impl Calendar for Calendars {
    fn is_marked(&self, date: NaiveDate) -> bool {
        self.calendars.iter().any(|c| c.is_marked(date))
    }

    fn mark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.mark(date));
    }

    fn unmark(&mut self, date: NaiveDate) {
        self.calendars.iter_mut().for_each(|c| c.unmark(date));
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        1 + self.height_list().sum::<usize>() + (self.calendars.len() / self.cols - 1)
    }
}

impl Display for Calendars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_line_width = self.width();

        writeln!(f, "{: ^width$}", self.title, width = max_line_width)?;

        let height_list: Vec<usize> = self.height_list().collect();

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

            for i in 0..height_list[line_count] {
                let mut line = String::new();

                for cal in &lines_list {
                    if let Some(cal_line) = cal.get(i) {
                        write!(&mut line, "{}{}", cal_line, " ".repeat(self.padding))?;
                    } else {
                        write!(&mut line, "{}", " ".repeat(cal[0].len() + self.padding))?;
                    }
                }

                for _ in 0..self.padding {
                    line.pop();
                }

                write!(f, "{}", line)?;

                if line_count < self.calendars.len() / self.cols - 1
                    || i != height_list[line_count] - 1
                {
                    writeln!(f)?;
                }
            }

            // カレンダーの間
            if line_count < self.calendars.len() / self.cols - 1 {
                writeln!(f, "{}", " ".repeat(max_line_width))?;
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
