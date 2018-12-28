const FIRST_VERSE : &'static str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const SECOND_VERSE : &'static str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";

pub fn verse(n: i32) -> String {
    match n {
        0 => FIRST_VERSE.to_string(),
        1 => SECOND_VERSE.to_string(),
        _ => format!(
            "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n",
            n,
            n,
            n-1,
            if n > 2 { "s" } else { "" }),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(|i| verse(i))
        .collect::<Vec<String>>()
        .join("\n")
}
