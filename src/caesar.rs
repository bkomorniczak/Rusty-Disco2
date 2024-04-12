pub fn cipher(input: &str, key: i32) -> String {
    input.to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let first_letter = b'A';
            let shifted = ((c as u8 - first_letter + (key.rem_euclid(26) as u8))).rem_euclid(26) % 26 + first_letter;
            shifted as char
        })
        .collect()
}