extern crate itertools;
use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        0
    } else {
        factors
            .iter()
            .filter_map(|factor| {
                if has_no_valid_multiples(limit, factor) {
                    None
                } else {
                    Some(multiples(limit, factor))
                }
            })
            .flatten()
            .unique()
            .sum()
    }
}

fn has_no_valid_multiples(limit: u32, factor: &u32) -> bool {
    limit < *factor || *factor == 0
}

fn multiples<'a>(limit: u32, factor: &'a u32) -> Box<Iterator<Item = u32> + 'a> {
    let upper = if limit % factor == 0 {
        (limit / factor) - 1
    } else {
        (limit / factor)
    };
    Box::new((0..upper).map(move |i| (i + 1) * *factor))
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn multiples_of_one() {
        assert_equal(Vec::<u32>::new().into_iter(), multiples(1, &1))
    }

    #[test]
    fn multiples_of_one_upto_3() {
        assert_equal(vec![1, 2].into_iter(), multiples(3, &1))
    }

    #[test]
    fn multiples_of_2_upto_5() {
        assert_equal(vec![2, 4].into_iter(), multiples(5, &2))
    }

    #[test]
    fn multiples_of_3_upto_10() {
        assert_equal(vec![3, 6, 9].into_iter(), multiples(10, &3))
    }

    #[test]
    fn multiples_of_5_upto_10() {
        assert_equal(vec![5].into_iter(), multiples(10, &5))
    }
}
