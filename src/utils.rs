#[macro_export]
macro_rules! n_date {
    ($year: expr, $month: expr, $day: expr) => {
        chrono::NaiveDate::from_ymd_opt($year, $month, $day).expect("Invalid date")
    };
}
