pub fn is_leap_year(year: i32) -> bool {
    fn is_divisible_by(number: i32, divisor: i32) -> bool {
        number % divisor == 0
    }
    is_divisible_by(year, 4) && (!is_divisible_by(year, 100) || is_divisible_by(year, 400))
}
