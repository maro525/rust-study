fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let len = calculate_length(&s);

    let word = first_word(&s[..]);

    println!(
        "The length of '{}' is {}. And the first letter is {}",
        s, len, word
    );

    s.clear();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
