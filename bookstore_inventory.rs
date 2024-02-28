use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    stock: u32,
}

enum Action {
    Add,
    Sell,
}

fn main() {
    let mut inventory: HashMap<String, Book> = HashMap::new();

    loop {
        println!("Please choose an action: Add, Sell, or Exit.");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");

        match action.trim().to_lowercase().as_str() {
            "add" => {
                let book = create_book();
                inventory.insert(book.title.clone(), book);
            },
            "sell" => {
                sell_book(&mut inventory);
            },
            "exit" => {
                println!("Exiting program.");
                break;
            },
            _ => println!("Invalid action."),
        }

        println!("Current Inventory: {:?}", inventory);
    }
}

fn create_book() -> Book {
    println!("Enter book title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");

    println!("Enter author:");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read line");

    println!("Enter stock quantity:");
    let mut stock_str = String::new();
    io::stdin().read_line(&mut stock_str).expect("Failed to read line");
    let stock: u32 = stock_str.trim().parse().expect("Please enter a number.");

    Book {
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        stock,
    }
}

fn sell_book(inventory: &mut HashMap<String, Book>) {
    println!("Enter the title of the book to sell:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim();

    if let Some(book) = inventory.get_mut(title) {
        if book.stock > 0 {
            book.stock -= 1;
            println!("Sold one copy of '{}'.", title);
        } else {
            println!("'{}' is out of stock.", title);
        }
    } else {
        println!("'{}' not found in inventory.", title);
    }
}
