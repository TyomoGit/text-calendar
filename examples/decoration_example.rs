use std::{fmt::Write, vec};

use chrono::{NaiveDate, Weekday};
use text_calendar::{BasicMarker, Calendar, Marker, MonthCalendar};

fn main() {
    example_builtin_marker();
    example_original_brackets();
}

fn example_builtin_marker() {
    let mut calendar = MonthCalendar::new(2024, 6, Weekday::Sun, 4, BasicMarker::UnderScore).unwrap();

    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 3).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 9).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 29).unwrap());

    println!("{}", calendar);
}

/*
            June            
 Su  Mo  Tu  We  Th  Fr  Sa 
                        _1 _
 2  _3 _ 4   5   6   7   8  
_9 _ 10  11  12  13 _14_ 15 
 16  17  18  19  20 _21_ 22 
 23  24  25  26  27  28 _29_
 30                         
 
 */

#[derive(Clone)]
struct BracketsMarker {
    day_width: usize,
    chars: Vec<char>,
}

impl Marker for BracketsMarker {
    fn decorate(&self, day: &str) -> String {
        let count = self.day_width - 2;

        let mut result = String::new();

        for _ in (0..count / 2).step_by(2) {
            write!(&mut result, "{}", self.chars[0]).unwrap();
        }

        write!(&mut result, "{}", day).unwrap();

        for _ in (0..count / 2).step_by(2) {
            write!(&mut result, "{}", self.chars[1]).unwrap();
        }

        if self.day_width % 2 == 1 {
            write!(&mut result, " ").unwrap();
        }
        result
    }
}

fn example_original_brackets() {
    let day_width = 2 + 2 * 2;
    let paren_marker = BracketsMarker { day_width, chars: vec!['<', '>' ] };
    let mut calendar = MonthCalendar::new(2024, 6, Weekday::Mon, 6, paren_marker).unwrap();

    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 3).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 9).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 29).unwrap());

    println!("{}", calendar);
}

/*
                   June                   
 Mon   Tue   Wed   Thu   Fri   Sat   Sun  
                               <1 >   2   
 <3 >   4     5     6     7     8    <9 > 
  10    11    12    13   <14>   15    16  
  17    18    19    20   <21>   22    23  
  24    25    26    27    28   <29>   30  

 */