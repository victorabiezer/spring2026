// Victor Chairez 
// March 05, 2026 

// REQUIREMENTS 

// Create a simple Book Catalog system in Rust 
// that demonstrates struct usage and file I/O operations.

// Create a Book struct with the following fields:
    // title: String
    // author: String
    // year: u16
    // Implement the following functions:

    // save_books(books: &Vec<Book>, filename: &str): Saves all books to a file.
    // load_books(filename: &str) -> Vec<Book>: Loads books from a file.
    // In the main function:

// Create a few Book instances
// Save the books to a file
// Load the books from the file and print them


// TASKS 
// Complete the save_books() function to write all books to a file. 
// Each book should be on a separate line with fields separated by commas.
// Implement the load_books() function to read books from the file and return a Vec<Book>.
// Run the program and verify that it correctly saves and loads the books.

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book 
{
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) 
{
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    // i need this function to write all books to a file 
    let mut file = File::create(filename).unwrap();
    for book in books
    {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> 
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut books = vec![];

    for line in reader.lines() 
    {  
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let book = Book 
        {
            title: parts[0].to_string(),
            author: parts[1].to_string(),
            year: parts[2].parse().unwrap(),
        };
        books.push(book);  
    }                     
    books                 
}


fn main() 
{
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books 
    {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}