pub fn build_proverb(list: &[&str]) -> String {
    let mut lines = String::new();
    if !list.is_empty() {
        let pairs = list.iter().zip(list.iter().skip(1));
        for (fst, sec) in pairs {
            lines.push_str(&format!("For want of a {} the {} was lost.\n", fst, sec));
        }
        lines.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    lines
}
