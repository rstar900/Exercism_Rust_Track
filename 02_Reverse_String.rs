pub fn reverse(input: &str) -> String {
    // Create an iterator by calling chars()
    // Reverse it using rev()
    // Put the result from iterator into a string by collect() or collect<String>()
    // TODO: Might need to check grapheme clusters to pass additional tests
    input.chars().rev().collect() 
}
