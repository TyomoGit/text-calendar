use chrono::Weekday;
use text_calendar::{BasicMarker, Calendar, Calendars, MonthCalendar, YearCalendar};

fn main() {
    example_month_cal();
    example_year_cal();
    example_original_multiple_calendars();
    example_calendars_in_calendars();
}

fn example_month_cal() {
    let calendar =
        MonthCalendar::new(2024, 2, Weekday::Sun, 4, BasicMarker::SquareBrackets).unwrap();

    println!("{}", calendar);
}

/*
          February          
 Su  Mo  Tu  We  Th  Fr  Sa 
                 1   2   3  
 4   5   6   7   8   9   10 
 11  12  13  14  15  16  17 
 18  19  20  21  22  23  24 
 25  26  27  28  29         

*/

fn example_year_cal() {
    let year_calendar = YearCalendar::new(2024, Weekday::Sun, 4, BasicMarker::SquareBrackets);

    println!("{}", year_calendar);
}

/*
                                                                                     2024                                            
          January                         February                         March            
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                       1   2   3                           1   2  
 7   8   9   10  11  12  13      4   5   6   7   8   9   10      3   4   5   6   7   8   9  
 14  15  16  17  18  19  20      11  12  13  14  15  16  17      10  11  12  13  14  15  16 
 21  22  23  24  25  26  27      18  19  20  21  22  23  24      17  18  19  20  21  22  23 
 28  29  30  31                  25  26  27  28  29              24  25  26  27  28  29  30 
                                                                 31                         
                                                                                            
           April                            May                             June            
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                   1   2   3   4                               1  
 7   8   9   10  11  12  13      5   6   7   8   9   10  11      2   3   4   5   6   7   8  
 14  15  16  17  18  19  20      12  13  14  15  16  17  18      9   10  11  12  13  14  15 
 21  22  23  24  25  26  27      19  20  21  22  23  24  25      16  17  18  19  20  21  22 
 28  29  30                      26  27  28  29  30  31          23  24  25  26  27  28  29 
                                                                 30                         
                                                                                            
            July                           August                        September          
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
     1   2   3   4   5   6                       1   2   3       1   2   3   4   5   6   7  
 7   8   9   10  11  12  13      4   5   6   7   8   9   10      8   9   10  11  12  13  14 
 14  15  16  17  18  19  20      11  12  13  14  15  16  17      15  16  17  18  19  20  21 
 21  22  23  24  25  26  27      18  19  20  21  22  23  24      22  23  24  25  26  27  28 
 28  29  30  31                  25  26  27  28  29  30  31      29  30                     
                                                                                            
          October                         November                        December          
 Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa      Su  Mo  Tu  We  Th  Fr  Sa 
         1   2   3   4   5                           1   2       1   2   3   4   5   6   7  
 6   7   8   9   10  11  12      3   4   5   6   7   8   9       8   9   10  11  12  13  14 
 13  14  15  16  17  18  19      10  11  12  13  14  15  16      15  16  17  18  19  20  21 
 20  21  22  23  24  25  26      17  18  19  20  21  22  23      22  23  24  25  26  27  28 
 27  28  29  30  31              24  25  26  27  28  29  30      29  30  31                 

*/

fn example_original_multiple_calendars() {
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

/*
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

*/

fn example_calendars_in_calendars() {
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

/*
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

 */