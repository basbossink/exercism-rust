pub fn build_proverb(list: &[&str]) -> String {
    let mut lines = String::new();
    if !list.is_empty() {
        let pairs = list.windows(2);
        for pair in pairs {
            lines.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                pair[0], pair[1]
            ));
        }
        lines.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    lines
}
