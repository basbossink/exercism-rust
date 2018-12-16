pub fn is_armstrong_number(num: u32) -> bool {
    let no_digits = number_of_digits(num);
    digits(num).iter().map(|d| d.pow(no_digits)).sum::<u32>() == num
}

fn digits(num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut rest = num;
    while 0 < rest {
        let remainder = rest % 10;
        rest = (rest - remainder) / 10;
        result.push(remainder);
    }
    result
}

fn number_of_digits(num: u32) -> u32 {
    ((num as f64).log10().floor() as u32) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_digits() {
        let cases = [
            (2, 1),
            (12, 2),
            (123, 3),
            (1234, 4),
            (12345, 5),
            (10, 2),
            (99, 2),
            (100, 3),
        ];
        for (arg, expected) in cases.iter() {
            assert_eq!(*expected, number_of_digits(*arg));
        }
    }

    #[test]
    fn test_digits() {
        let cases = [(2, vec![2]), (12, vec![2, 1]), (123, vec![3, 2, 1])];
        for (arg, expected) in cases.iter() {
            assert_eq!(*expected, digits(*arg));
        }
    }
}
