use std::io;

fn main() {
    // Step 1: Get patient information
    let mut input = String::new();

    println!("Enter patient name: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let name = input.trim().to_string();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let age: u32 = input.trim().parse().expect("Please enter a valid number for age");
    input.clear();

    println!("Enter number of children: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_children: u32 = input.trim().parse().expect("Please enter a valid number for children");
    input.clear();

    println!("Enter number of siblings: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_siblings: u32 = input.trim().parse().expect("Please enter a valid number for siblings");
    input.clear();

    println!("Enter diagnosis (Alzheimer, Arrhythmia, CKD, Diabetes, Arthritis): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let diagnosis = input.trim().to_string();
    input.clear();

    println!("Enter village: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let village = input.trim().to_string();
    input.clear();

    // Step 2: Set the base amount and discount based on diagnosis
    let mut amount: f32 = 0.0;
    let mut discount: f32 = 0.0;

    if diagnosis == "Alzheimer" {
        amount = 1_200_000.0;
        if age > 50 && num_children > 4 && village == "Akpabom" {
            discount = 0.20;
        }
    } else if diagnosis == "Arrhythmia" {
        amount = 550_000.0;
        if age == 30 && num_siblings > 4 && village == "Ngbuaji" {
            discount = 0.05;
        }
    } else if diagnosis == "CKD" {
        amount = 1_000_000.0;
        if age > 40 && num_children > 3 && village == "Atabarikang" {
            discount = 0.15;
        }
    } else if diagnosis == "Diabetes" {
        amount = 800_000.0;
        if age > 28 && age < 45 && num_children >= 2 && num_children <= 4 && village == "Okorobiom" {
            discount = 0.10;
        }
    } else if diagnosis == "Arthritis" {
        amount = 450_000.0;
        if age > 58 && num_siblings > 5 && village == "Emeremen" {
            discount = 0.10;
        }
    } else {
        println!("Unknown diagnosis entered.");
        return;
    }

    // Step 3: Calculate final amount
    let final_amount = amount * (1.0 - discount);
    println!("{}'s final charge is: ₦{:.2}", name, final_amount);
}
