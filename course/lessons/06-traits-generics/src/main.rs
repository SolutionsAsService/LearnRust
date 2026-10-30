//! Lesson 06: traits, generics and trait objects.

trait Describe {
    fn title(&self) -> String;

    /// A default method: types get it for free and may override it.
    fn describe(&self) -> String {
        format!("[{}]", self.title())
    }
}

struct Book {
    name: String,
    pages: u32,
}

struct Film {
    name: String,
    minutes: u32,
}

impl Describe for Book {
    fn title(&self) -> String {
        self.name.clone()
    }
    fn describe(&self) -> String {
        format!("{} ({} pages)", self.name, self.pages)
    }
}

impl Describe for Film {
    fn title(&self) -> String {
        self.name.clone()
    }
}

/// Generic over any type that can be compared and copied.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let first = iter.next()?;
    Some(iter.fold(first, |a, b| if b > a { b } else { a }))
}

/// Static dispatch: compiled once per concrete type.
fn shout<T: Describe>(item: &T) -> String {
    item.describe().to_uppercase()
}

fn main() {
    // Dynamic dispatch: one Vec of different types behind `dyn Describe`.
    let shelf: Vec<Box<dyn Describe>> = vec![
        Box::new(Book { name: "Rust in Action".into(), pages: 456 }),
        Box::new(Film { name: "Hackers".into(), minutes: 105 }),
    ];
    for item in &shelf {
        println!("{}", item.describe());
    }
    println!("{}", shout(&Book { name: "Zero to Production".into(), pages: 500 }));
    println!("largest: {:?} {:?}", largest(&[3, 9, 2]), largest(&[1.5, -2.0]));
    let film = Film { name: "Sneakers".into(), minutes: 126 };
    println!("{} runs {} minutes", film.title(), film.minutes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_works_for_ints_floats_and_empty() {
        assert_eq!(largest(&[1, 5, 3]), Some(5));
        assert_eq!(largest(&[0.5, 0.25]), Some(0.5));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn default_and_overridden_methods() {
        assert_eq!(Film { name: "X".into(), minutes: 1 }.describe(), "[X]");
        assert_eq!(Book { name: "Y".into(), pages: 2 }.describe(), "Y (2 pages)");
    }
}
