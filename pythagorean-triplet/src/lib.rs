extern crate num_integer;
use num_integer::Integer;
use std::collections::HashSet;

// Euclid's formula
// a = k(m**2 - n**2)
// b = k2mn
// c = k(m**2 + n**2)
// The exercise states that
// sum = a + b + c
// sum = m**2 - n**2 + 2mn + m**2 + n**2
// sum = 2m**2 + 2mn
// sum = 2m(m + n)
// sum/2 = m(m + n)
// sum/2m = m + n
// sum/2m - m = n
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    if sum.is_odd() {
        result
    } else {
        let upper_k = f64::from(sum).sqrt().floor() as u32;
        for k in 1..(upper_k + 1) {
            let (divident, remainder) = sum.div_rem(&k);
            if remainder == 0 {
                primiteve_triples(divident, k, &mut result);
            }
        }
        result
    }
}

fn primiteve_triples(sum: u32, multiplier: u32, result: &mut HashSet<[u32; 3]>) {
    let to_divide = sum / 2;
    let upper = f64::from(to_divide).sqrt().floor() as u32;
    for m in 1..(upper + 1) {
        let (divident, remainder) = to_divide.div_rem(&m);
        if remainder == 0 {
            let n = divident - m;
            if m > n {
                let m_sq = m * m;
                let n_sq = n * n;
                let a = multiplier * (m_sq - n_sq);
                let b = multiplier * 2 * m * n;
                let c = multiplier * (m_sq + n_sq);
                if (a + b + c == (multiplier * sum)) && (a * a + b * b == c * c) {
                    let triple = if a <= b { [a, b, c] } else { [b, a, c] };
                    if a != 0 && b != 0 {
                        result.insert(triple);
                    }
                }
            }
        }
    }
}
