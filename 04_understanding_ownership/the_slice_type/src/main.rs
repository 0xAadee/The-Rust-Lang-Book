fn main() {
    // Slices let you reference contiguous sequence of values in a collections instead of the entire collection
    // Slices don't take the ownership of the underlying data

    let s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}