//! Lesson 02: ownership, moves and borrows.

/// Count words by *borrowing* the text: the caller keeps ownership.
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Return a slice of the input. The lifetime `'a` says the result lives as long as `text`.
fn longest_word<'a>(text: &'a str) -> &'a str {
    text.split_whitespace().fold("", |best, w| if w.len() > best.len() { w } else { best })
}

/// Take ownership and give back a new String: the original is consumed.
fn shout(mut text: String) -> String {
    text.make_ascii_uppercase();
    text.push('!');
    text
}

fn main() {
    let s = String::from("ownership is the heart of rust");

    // Borrow: `s` is still usable afterwards.
    println!("{} words, longest is {:?}", word_count(&s), longest_word(&s));

    // Clone when you really need two owners of the data.
    let copy = s.clone();

    // Move: `s` is given away. Using `s` after this line will not compile:
    // error[E0382]: borrow of moved value: `s`
    let loud = shout(s);
    println!("{loud}");
    println!("the clone still works: {copy}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        assert_eq!(word_count("one two  three"), 3);
    }

    #[test]
    fn finds_longest_and_keeps_the_owner() {
        let text = String::from("a borrow is fine");
        assert_eq!(longest_word(&text), "borrow");
        assert_eq!(text.len(), 16); // still ours
    }

    #[test]
    fn shout_consumes_and_returns() {
        assert_eq!(shout("hi".to_string()), "HI!");
    }
}
