use std::io;

fn checker() {
    let mut input = String::new(); // Fixing declaration of String
    println!("Enter a character:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input
    let ch: char = input.trim().chars().next().unwrap_or('\0'); // Safely get the first character

    // Check if the character is a digit
    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit", ch);
    } else {
        println!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    // Calling the function
    println!("Welcome! This program checks whether a character variable contains a digit or not");
    checker();
}
