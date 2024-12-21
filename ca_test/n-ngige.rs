use std::io::{self, Write};

fn main() {
    print!("Enter the number of siblings: ");
    io::stdout().flush().unwrap();
    let mut num_siblings = String::new();
    io::stdin().read_line(&mut num_siblings).unwrap();
    let num_siblings: u32 = num_siblings.trim().parse().unwrap();

    for i in 0..num_siblings {
        println!("\nDetails for sibling #{}", i + 1);

        print!("Enter first name: ");
        io::stdout().flush().unwrap();
        let mut first_name = String::new();
        io::stdin().read_line(&mut first_name).unwrap();

        print!("Enter age: ");
        io::stdout().flush().unwrap();
        let mut age = String::new();
        io::stdin().read_line(&mut age).unwrap();
        let age: u32 = age.trim().parse().unwrap();

        print!("Enter gender: ");
        io::stdout().flush().unwrap();
        let mut gender = String::new();
        io::stdin().read_line(&mut gender).unwrap();

        print!("Enter country of residence: ");
        io::stdout().flush().unwrap();
        let mut country = String::new();
        io::stdin().read_line(&mut country).unwrap();

        if age >= 18 {
            print!("Is the sibling married, single, or in a relationship? ");
            io::stdout().flush().unwrap();
            let mut status = String::new();
            io::stdin().read_line(&mut status).unwrap();
            let status = status.trim().to_lowercase();

            match status.as_str() {
                "married" => {
                    // Collect and display details about children
                    // Example: Collect child name, age, and school
                    println!("Collecting details about children...");
                }
                "single" => {
                    print!("Is the sibling a student or employed? ");
                    io::stdout().flush().unwrap();
                    let mut occupation = String::new();
                    io::stdin().read_line(&mut occupation).unwrap();
                    let occupation = occupation.trim().to_lowercase();

                    if occupation == "student" {
                        // Collect and display student details
                        println!("Collecting university and course details...");
                    } else if occupation == "employed" {
                        // Collect and display employment details
                        println!("Collecting job title and industry sector...");
                    }
                }
                "in a relationship" => {
                    // Collect relationship details
                    println!("Collecting relationship duration and partner name...");
                }
                _ => println!("Invalid status entered."),
            }
        } else {
            print!("Has the sibling completed WAEC? (yes/no): ");
            io::stdout().flush().unwrap();
            let mut waec_status = String::new();
            io::stdin().read_line(&mut waec_status).unwrap();
            let waec_status = waec_status.trim().to_lowercase();

            if waec_status == "yes" {
                // Collect and display WAEC details
                println!("Collecting WAEC details...");
            } else {
                // Ask for current class level
                println!("Asking about class level and WAEC plans...");
            }
        }
    }
}