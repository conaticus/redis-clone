// Extracts the first char in a string and returns string without that char
pub fn extract_first_char(string: &str) -> (char, String) {
    let mut chars = string.chars();
    let first_char = chars
        .next()
        .expect("Could not extract character because string is empty");

    (first_char, chars.collect())
}
