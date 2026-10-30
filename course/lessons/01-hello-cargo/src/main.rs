//! Lesson 01: your first Cargo project.

/// Build a greeting. Returning a `String` (instead of printing) makes it testable.
fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to Rust.")
}

fn main() {
    let learner = "Ferris";
    println!("{}", greet(learner));

    // `{:>8}` right-aligns in 8 columns, `{:.2}` keeps two decimals.
    let price = 3.14159_f64;
    println!("[{:>8}] [{:.2}]", "right", price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet("Ada"), "Hello, Ada! Welcome to Rust.");
    }
}
