// find the first word from a string if string only has one word return the word;

pub fn find_first_word(string: &String) -> &str {
    let bytes = string.as_bytes();

    for (index, &sting_char) in bytes.iter().enumerate() {
        if sting_char == b' ' {
            return &string[..index];
        }
    }

    &string[..]
}