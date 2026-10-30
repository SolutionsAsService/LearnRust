//! Lesson 04: a bookstore inventory with `HashMap`, `Vec` and `Result`.
//! Grown from the repository's original `bookstore_inventory.rs`.

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    stock: u32,
}

#[derive(Debug, PartialEq)]
enum SellError {
    NotFound,
    OutOfStock,
}

#[derive(Default)]
struct Inventory {
    books: HashMap<String, Book>,
}

impl Inventory {
    /// Add copies; a title that already exists gets its stock topped up.
    fn add(&mut self, title: &str, author: &str, copies: u32) {
        self.books
            .entry(title.to_lowercase())
            .and_modify(|b| b.stock += copies)
            .or_insert(Book { title: title.into(), author: author.into(), stock: copies });
    }

    fn sell(&mut self, title: &str) -> Result<u32, SellError> {
        let book = self.books.get_mut(&title.to_lowercase()).ok_or(SellError::NotFound)?;
        if book.stock == 0 {
            return Err(SellError::OutOfStock);
        }
        book.stock -= 1;
        Ok(book.stock)
    }

    /// Books sorted by title, for stable output.
    fn list(&self) -> Vec<&Book> {
        let mut v: Vec<&Book> = self.books.values().collect();
        v.sort_by(|a, b| a.title.cmp(&b.title));
        v
    }
}

fn main() {
    let mut inv = Inventory::default();
    inv.add("The Rust Programming Language", "Klabnik & Nichols", 3);
    let stdin = io::stdin();
    println!("Commands: add <title> | <author> | <copies>, sell <title>, list, exit");
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            break; // end of input
        }
        let line = line.trim();
        let (cmd, rest) = line.split_once(' ').unwrap_or((line, ""));
        match cmd {
            "add" => {
                let parts: Vec<&str> = rest.split('|').map(str::trim).collect();
                match parts.as_slice() {
                    [title, author, copies] => match copies.parse::<u32>() {
                        Ok(n) => inv.add(title, author, n),
                        Err(_) => println!("copies must be a whole number"),
                    },
                    _ => println!("usage: add <title> | <author> | <copies>"),
                }
            }
            "sell" => match inv.sell(rest) {
                Ok(left) => println!("sold one, {left} left"),
                Err(SellError::NotFound) => println!("not in the inventory"),
                Err(SellError::OutOfStock) => println!("out of stock"),
            },
            "list" => {
                for b in inv.list() {
                    println!("{:<40} {:<20} {:>3}", b.title, b.author, b.stock);
                }
            }
            "exit" => break,
            "" => {}
            other => println!("unknown command: {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_then_sell_down_to_zero() {
        let mut inv = Inventory::default();
        inv.add("Dune", "Herbert", 1);
        assert_eq!(inv.sell("dune"), Ok(0));
        assert_eq!(inv.sell("Dune"), Err(SellError::OutOfStock));
    }

    #[test]
    fn unknown_title() {
        assert_eq!(Inventory::default().sell("Nope"), Err(SellError::NotFound));
    }

    #[test]
    fn adding_again_tops_up() {
        let mut inv = Inventory::default();
        inv.add("Dune", "Herbert", 1);
        inv.add("dune", "Herbert", 2);
        assert_eq!(inv.list()[0].stock, 3);
    }
}
