pub fn raindrops(n: usize) -> String {
    fn has_factor(number: usize, divisor: usize) -> bool {
        number % divisor == 0
    }

    match n {
        x if has_factor(x, 3 * 5 * 7) => "PlingPlangPlong".to_string(),
        x if has_factor(x, 5 * 7) => "PlangPlong".to_string(),
        x if has_factor(x, 3 * 7) => "PlingPlong".to_string(),
        x if has_factor(x, 3 * 5) => "PlingPlang".to_string(),
        x if has_factor(x, 7) => "Plong".to_string(),
        x if has_factor(x, 5) => "Plang".to_string(),
        x if has_factor(x, 3) => "Pling".to_string(),
        x => x.to_string(),
    }
}
