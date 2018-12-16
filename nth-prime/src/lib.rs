pub fn nth(n: u32) -> Option<u32> {
    match n {
        n if n < 1 => None,
        _ => Some(primes_upto(upper_bound(n))[(n - 1) as usize]),
    }
}

fn upper_bound(n: u32) -> usize {
    if n <= 6 {
        17
    } else {
        let n_float = n as f64;
        (n_float * (n_float * n_float.ln()).ln()).ceil() as usize
    }
}

fn primes_upto(n: usize) -> Vec<u32> {
    let mut sieve = vec![true; n + 1usize];
    for i in 2..bound(n) {
        if sieve[i] {
            for j in multiples(i, n) {
                sieve[j] = false;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter_map(|(index, val)| if *val { Some(index as u32) } else { None })
        .skip(2)
        .collect()
}

fn bound(n: usize) -> usize {
    (n as f64).sqrt().ceil() as usize
}

fn multiples(num: usize, bound: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut multiple = num * num;
    while multiple <= bound {
        result.push(multiple as usize);
        multiple = multiple + num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples() {
        let cases = [
            (2, 10, vec![4, 6, 8, 10]),
            (3, 20, vec![9, 12, 15, 18]),
            (5, 47, vec![25, 30, 35, 40, 45]),
        ];
        for (num, bound, expected) in cases.iter() {
            assert_eq!(*expected, multiples(*num, *bound));
        }
    }

    #[test]
    fn test_sieve() {
        let expected = vec![2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(expected, primes_upto(30));
    }
}
