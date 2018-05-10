pub fn reverse(input: &str) -> String {
    fn reverse_recursively(input: &str, accumulator: String) -> String {
        let mut iterator = input.chars();
        match iterator.next() {
            Some(character) => reverse_recursively(
                &iterator.collect::<String>(),
                character.to_string() + &accumulator,
            ),
            None => accumulator,
        }
    }
    reverse_recursively(input, String::new())
}
