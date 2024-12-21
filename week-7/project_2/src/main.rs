use std::io;

fn main() {
    // Define a vector to store candidates' details (name and years of experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Get the number of candidates
    let mut input = String::new();
    println!("Enter the number of candidates:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num_candidates: usize = input.trim().parse().expect("Invalid input");

    // Collect candidates' details
    for i in 1..=num_candidates {
        // Get candidate's name
        let mut name = String::new();
        println!("Enter the name of candidate {}:", i);
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let name = name.trim().to_string();

        // Get candidate's years of programming experience
        let mut experience = String::new();
        println!("Enter the years of programming experience for {}:", name);
        io::stdin()
            .read_line(&mut experience)
            .expect("Failed to read input");
        let experience: u32 = experience.trim().parse().expect("Invalid input");

        // Add the candidate to the vector
        candidates.push((name, experience));
    }

    // Find the candidate with the highest experience
    let mut most_experienced = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > most_experienced.1 {
            most_experienced = candidate;
        }
    }

    // Display the result
    println!(
        "\nThe most experienced candidate is {} with {} years of programming experience.",
        most_experienced.0, most_experienced.1
    );
}