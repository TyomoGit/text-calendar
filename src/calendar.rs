use chrono::NaiveDate;
use std::fmt::{Debug, Display};

pub trait Calendar: Display {
    fn mark(&mut self, date: NaiveDate);
    fn unmark(&mut self, date: NaiveDate);
    fn is_marked(&self, date: NaiveDate) -> bool;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl Debug for dyn Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}