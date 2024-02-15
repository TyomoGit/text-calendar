# text-calendar
A Rust crate for generating text-based calendars.

## Description

This crate provides a simple way to generate text-based calendars.


## Usage

There are two calendars are built-in: `MonthCalendar` and `YearCalendar`.

### MonthCalendar

`MonthCalendar` provides a calendar for specific month and year.

```rust
fn main() {
    let calendar =
        MonthCalendar::new(2024, 2, Weekday::Sun, 4, BasicMarker::SquareBrackets).unwrap();

    println!("{}", calendar);
}
```
output:
```text
          February          
 Su  Mo  Tu  We  Th  Fr  Sa 
                 1   2   3  
 4   5   6   7   8   9   10 
 11  12  13  14  15  16  17 
 18  19  20  21  22  23  24 
 25  26  27  28  29         

```

### YearCalendar
`YearCalendar` provides a calendar for specific year. It has 12 months calendars.

```rust
fn main() {
    let year_calendar = YearCalendar::new(2024, Weekday::Sun, 4, BasicMarker::SquareBrackets);

    println!("{}", year_calendar);
}
```
output:
```text
                                         2024                                           
          January                       February                       March            
 Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                     1   2   3                         1   2  
 7   8   9   10  11  12  13    4   5   6   7   8   9   10    3   4   5   6   7   8   9  
 14  15  16  17  18  19  20    11  12  13  14  15  16  17    10  11  12  13  14  15  16 
 21  22  23  24  25  26  27    18  19  20  21  22  23  24    17  18  19  20  21  22  23 
 28  29  30  31                25  26  27  28  29            24  25  26  27  28  29  30 
                                                             31                         

           April                          May                           June            
 Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                 1   2   3   4                             1  
 7   8   9   10  11  12  13    5   6   7   8   9   10  11    2   3   4   5   6   7   8  
 14  15  16  17  18  19  20    12  13  14  15  16  17  18    9   10  11  12  13  14  15 
 21  22  23  24  25  26  27    19  20  21  22  23  24  25    16  17  18  19  20  21  22 
 28  29  30                    26  27  28  29  30  31        23  24  25  26  27  28  29 
                                                             30                         

            July                         August                      September          
 Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                     1   2   3     1   2   3   4   5   6   7  
 7   8   9   10  11  12  13    4   5   6   7   8   9   10    8   9   10  11  12  13  14 
 14  15  16  17  18  19  20    11  12  13  14  15  16  17    15  16  17  18  19  20  21 
 21  22  23  24  25  26  27    18  19  20  21  22  23  24    22  23  24  25  26  27  28 
 28  29  30  31                25  26  27  28  29  30  31    29  30                     

          October                       November                      December          
 Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa    Su  Mo  Tu  We  Th  Fr  Sa 
         1   2   3   4   5                         1   2     1   2   3   4   5   6   7  
 6   7   8   9   10  11  12    3   4   5   6   7   8   9     8   9   10  11  12  13  14 
 13  14  15  16  17  18  19    10  11  12  13  14  15  16    15  16  17  18  19  20  21 
 20  21  22  23  24  25  26    17  18  19  20  21  22  23    22  23  24  25  26  27  28 
 27  28  29  30  31            24  25  26  27  28  29  30    29  30  31                 

```

You can create your own multiple month calendar by using `Calendars` struct. Actually, `YearCalendar` is a wrapper for this struct.

```rust
fn main() {
    let months: Vec<u32> = (1..=12).rev().collect();

    let calendar_list: Vec<Box<dyn Calendar>> = months
        .iter()
        .map(|month| -> Box<dyn Calendar> {
            Box::new(
                MonthCalendar::new(2024, *month, Weekday::Sun, 8, BasicMarker::SquareBrackets)
                    .unwrap(),
            )
        })
        .collect();

    let calendars = Calendars::new(calendar_list, "Reversed Calendar".to_string(), 5);
    println!("{}", calendars);
}
```
output:
```text
                                                                                                                                                   Reversed Calendar                                                                                                                                                    
                        December                                                        November                                                        October                                                        September                                                         August                         
  Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat   
   1       2       3       4       5       6       7                                                       1       2                               1       2       3       4       5               1       2       3       4       5       6       7                                               1       2       3    
   8       9       10      11      12      13      14              3       4       5       6       7       8       9               6       7       8       9       10      11      12              8       9       10      11      12      13      14              4       5       6       7       8       9       10   
   15      16      17      18      19      20      21              10      11      12      13      14      15      16              13      14      15      16      17      18      19              15      16      17      18      19      20      21              11      12      13      14      15      16      17   
   22      23      24      25      26      27      28              17      18      19      20      21      22      23              20      21      22      23      24      25      26              22      23      24      25      26      27      28              18      19      20      21      22      23      24   
   29      30      31                                              24      25      26      27      28      29      30              27      28      29      30      31                              29      30                                                      25      26      27      28      29      30      31   
                                                                                                                                                                                                                                                                                                                        
                          July                                                            June                                                            May                                                            April                                                           March                          
  Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat   
           1       2       3       4       5       6                                                               1                                       1       2       3       4                       1       2       3       4       5       6                                                       1       2    
   7       8       9       10      11      12      13              2       3       4       5       6       7       8               5       6       7       8       9       10      11              7       8       9       10      11      12      13              3       4       5       6       7       8       9    
   14      15      16      17      18      19      20              9       10      11      12      13      14      15              12      13      14      15      16      17      18              14      15      16      17      18      19      20              10      11      12      13      14      15      16   
   21      22      23      24      25      26      27              16      17      18      19      20      21      22              19      20      21      22      23      24      25              21      22      23      24      25      26      27              17      18      19      20      21      22      23   
   28      29      30      31                                      23      24      25      26      27      28      29              26      27      28      29      30      31                      28      29      30                                              24      25      26      27      28      29      30   
                                                                   30                                                                                                                                                                                              31                                                   
                                                                                                                                                                                                                                                                                                                        
                        February                                                        January                                                                                                                                                                                                                         
  Sun     Mon     Tue     Wed     Thu     Fri     Sat             Sun     Mon     Tue     Wed     Thu     Fri     Sat                                                                                                                                                                                                   
                                   1       2       3                       1       2       3       4       5       6                                                                                                                                                                                                    
   4       5       6       7       8       9       10              7       8       9       10      11      12      13                                                                                                                                                                                                   
   11      12      13      14      15      16      17              14      15      16      17      18      19      20                                                                                                                                                                                                   
   18      19      20      21      22      23      24              21      22      23      24      25      26      27                                                                                                                                                                                                   
   25      26      27      28      29                              28      29      30      31                                                                                                                                                                                                                           

```

**`YearCalendar` and `Calendars` also implement the `Calendar` trait.** Therefore, `Calendars` can have multiple `YearCalendars` as `dyn Calendar`.

```rust
fn main() {
    let cal2024 = YearCalendar::new(2024, Weekday::Sun, 4, BasicMarker::SquareBrackets);
    let cal2025 = YearCalendar::new(2025, Weekday::Sun, 4, BasicMarker::SquareBrackets);
    let cal2026 = YearCalendar::new(2026, Weekday::Sun, 4, BasicMarker::SquareBrackets);

    let calendars = Calendars::new(
        vec![Box::new(cal2024), Box::new(cal2025), Box::new(cal2026)],
        String::from("24,25,26 Calendar"),
        3,
    );

    println!("{}", calendars);
}
```
output:
```text
                                                                                                                                              24,25,26 Calendar                                                                                                                                               
                                            2024                                                                                                     2025                                                                                                     2026                                            
          January                         February                         March                                   January                         February                         March                                   January                         February                         March            
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                       1   2   3                           1   2                            1   2   3   4                               1                               1                                1   2   3       1   2   3   4   5   6   7       1   2   3   4   5   6   7  
 7   8   9   10  11  12  13      4   5   6   7   8   9   10      3   4   5   6   7   8   9                5   6   7   8   9   10  11      2   3   4   5   6   7   8       2   3   4   5   6   7   8                4   5   6   7   8   9   10      8   9   10  11  12  13  14      8   9   10  11  12  13  14 
 14  15  16  17  18  19  20      11  12  13  14  15  16  17      10  11  12  13  14  15  16               12  13  14  15  16  17  18      9   10  11  12  13  14  15      9   10  11  12  13  14  15               11  12  13  14  15  16  17      15  16  17  18  19  20  21      15  16  17  18  19  20  21 
 21  22  23  24  25  26  27      18  19  20  21  22  23  24      17  18  19  20  21  22  23               19  20  21  22  23  24  25      16  17  18  19  20  21  22      16  17  18  19  20  21  22               18  19  20  21  22  23  24      22  23  24  25  26  27  28      22  23  24  25  26  27  28 
 28  29  30  31                  25  26  27  28  29              24  25  26  27  28  29  30               26  27  28  29  30  31          23  24  25  26  27  28          23  24  25  26  27  28  29               25  26  27  28  29  30  31                                      29  30  31                 
                                                                 31                                                                                                       30  31                                                                                                                              
                                                                                                                                                                                                                             April                            May                             June            
           April                            May                             June                                    April                            May                             June                          Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa                           1   2   3   4                           1   2           1   2   3   4   5   6  
     1   2   3   4   5   6                   1   2   3   4                               1                        1   2   3   4   5                       1   2   3       1   2   3   4   5   6   7                5   6   7   8   9   10  11      3   4   5   6   7   8   9       7   8   9   10  11  12  13 
 7   8   9   10  11  12  13      5   6   7   8   9   10  11      2   3   4   5   6   7   8                6   7   8   9   10  11  12      4   5   6   7   8   9   10      8   9   10  11  12  13  14               12  13  14  15  16  17  18      10  11  12  13  14  15  16      14  15  16  17  18  19  20 
 14  15  16  17  18  19  20      12  13  14  15  16  17  18      9   10  11  12  13  14  15               13  14  15  16  17  18  19      11  12  13  14  15  16  17      15  16  17  18  19  20  21               19  20  21  22  23  24  25      17  18  19  20  21  22  23      21  22  23  24  25  26  27 
 21  22  23  24  25  26  27      19  20  21  22  23  24  25      16  17  18  19  20  21  22               20  21  22  23  24  25  26      18  19  20  21  22  23  24      22  23  24  25  26  27  28               26  27  28  29  30              24  25  26  27  28  29  30      28  29  30                 
 28  29  30                      26  27  28  29  30  31          23  24  25  26  27  28  29               27  28  29  30                  25  26  27  28  29  30  31      29  30                                                                   31                                                         
                                                                 30                                                                                                                                                                                                                                           
                                                                                                                     July                           August                        September                                   July                           August                        September          
            July                           August                        September                        Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa                       1   2   3   4   5                           1   2           1   2   3   4   5   6                            1   2   3   4                               1               1   2   3   4   5  
     1   2   3   4   5   6                       1   2   3       1   2   3   4   5   6   7                6   7   8   9   10  11  12      3   4   5   6   7   8   9       7   8   9   10  11  12  13               5   6   7   8   9   10  11      2   3   4   5   6   7   8       6   7   8   9   10  11  12 
 7   8   9   10  11  12  13      4   5   6   7   8   9   10      8   9   10  11  12  13  14               13  14  15  16  17  18  19      10  11  12  13  14  15  16      14  15  16  17  18  19  20               12  13  14  15  16  17  18      9   10  11  12  13  14  15      13  14  15  16  17  18  19 
 14  15  16  17  18  19  20      11  12  13  14  15  16  17      15  16  17  18  19  20  21               20  21  22  23  24  25  26      17  18  19  20  21  22  23      21  22  23  24  25  26  27               19  20  21  22  23  24  25      16  17  18  19  20  21  22      20  21  22  23  24  25  26 
 21  22  23  24  25  26  27      18  19  20  21  22  23  24      22  23  24  25  26  27  28               27  28  29  30  31              24  25  26  27  28  29  30      28  29  30                               26  27  28  29  30  31          23  24  25  26  27  28  29      27  28  29  30             
 28  29  30  31                  25  26  27  28  29  30  31      29  30                                                                   31                                                                                                       30  31                                                     
                                                                                                                                                                                                                                                                                                              
          October                         November                        December                                 October                         November                        December                                 October                         November                        December          
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa               Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
         1   2   3   4   5                           1   2       1   2   3   4   5   6   7                            1   2   3   4                               1           1   2   3   4   5   6                                1   2   3       1   2   3   4   5   6   7               1   2   3   4   5  
 6   7   8   9   10  11  12      3   4   5   6   7   8   9       8   9   10  11  12  13  14               5   6   7   8   9   10  11      2   3   4   5   6   7   8       7   8   9   10  11  12  13               4   5   6   7   8   9   10      8   9   10  11  12  13  14      6   7   8   9   10  11  12 
 13  14  15  16  17  18  19      10  11  12  13  14  15  16      15  16  17  18  19  20  21               12  13  14  15  16  17  18      9   10  11  12  13  14  15      14  15  16  17  18  19  20               11  12  13  14  15  16  17      15  16  17  18  19  20  21      13  14  15  16  17  18  19 
 20  21  22  23  24  25  26      17  18  19  20  21  22  23      22  23  24  25  26  27  28               19  20  21  22  23  24  25      16  17  18  19  20  21  22      21  22  23  24  25  26  27               18  19  20  21  22  23  24      22  23  24  25  26  27  28      20  21  22  23  24  25  26 
 27  28  29  30  31              24  25  26  27  28  29  30      29  30  31                               26  27  28  29  30  31          23  24  25  26  27  28  29      28  29  30  31                           25  26  27  28  29  30  31      29  30                          27  28  29  30  31         
                                                                                                                                          30                                                                                                                                                                  

```


### Marker

`Marker` provides a marker, which is used to mark specific day.

```rust
fn main() {
    let mut calendar = MonthCalendar::new(2024, 6, Weekday::Sun, 4, BasicMarker::UnderScore).unwrap();

    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 3).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 9).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 21).unwrap());
    calendar.mark(NaiveDate::from_ymd_opt(2024, 6, 29).unwrap());

    println!("{}", calendar);
}
```
In this example, `BasicMarker::UnderScore` is used to mark the specific day.
This marker marks the day with `_` such as `_12_`.
output:
```text
            June            
 Su  Mo  Tu  We  Th  Fr  Sa 
                        _1 _
 2  _3 _ 4   5   6   7   8  
_9 _ 10  11  12  13 _14_ 15 
 16  17  18  19  20 _21_ 22 
 23  24  25  26  27  28 _29_
 30                         
 
```

There are some other markers:
```rust
pub enum BasicMarker {
    None,
    SquareBrackets,
    UnderScore,
    Char(char),
}
```

You can create your own marker by implementing `Marker` trait.

```rust
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

fn main() {
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
```

```text
                   June                   
 Mon   Tue   Wed   Thu   Fri   Sat   Sun  
                               <1 >   2   
 <3 >   4     5     6     7     8    <9 > 
  10    11    12    13   <14>   15    16  
  17    18    19    20   <21>   22    23  
  24    25    26    27    28   <29>   30  

```