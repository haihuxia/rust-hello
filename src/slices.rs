fn first() {
    // only UTF-8 character
    let s = String::from("hello world");
    let len = s.len();

    let hello = &s[0..5];
    let hello = &s[0..=4];
    let hello = &s[..5];

    let world = &s[6..11];
    let world = &s[6..=10];
    let world = &s[6..len];
    let world = &s[6..];

    let hello_world = &s[0..len];
    let hello_world = &s[..];
}

fn first_word_a(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_b(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use crate::slices;

    #[test]
    fn a() {
        let hello = String::from("hello world");

        let word = slices::first_word_a(&hello);

        assert_eq!(5, word);
    }

    #[test]
    fn b() {
        let hello = String::from("hello world");

        let word = slices::first_word_b(&hello);

        assert_eq!("hello", word);
    }

    #[test]
    fn c() {
        let hello = "hello world";

        let word = slices::first_word_b(&hello);

        assert_eq!("hello", word);
    }
}
