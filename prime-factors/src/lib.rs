pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut rest = n;
    let mut divisor = 2;
    while divisor <= rest {
        while divides(rest, divisor) {
            result.push(divisor);
            rest = rest / divisor;
        }
        divisor = divisor + 1
    }
    result
}

fn divides(lhs: u64, rhs: u64) -> bool {
    lhs % rhs == 0
}
