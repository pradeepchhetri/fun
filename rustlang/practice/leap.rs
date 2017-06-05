/// on every year that is evenly divisible by 4
///   except every year that is evenly divisible by 100
///     unless the year is also evenly divisible by 400

pub fn is_leap_year(year: i64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false
    }
}