use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    for a in 1..sum / 2 {
        let a_squared = a * a;
        for b in a + 1..((sum - a) / 2) + 1 {
            let sum_of_sides = a_squared + b * b;
            let c = sum - a - b;
            if sum_of_sides == c * c {
                result.insert([a, b, c]);
            }
        }
    }
    result
}
