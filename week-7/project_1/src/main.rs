use std::io;

fn main() {
    // Vectors to store roles and their APS levels
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL 1 8-10", "EL 2 10-13", "SES"];
    let roles = vec![
        vec!["Intern", "Paralegal", "Placement"],             // APS 1-2
        vec!["Administrator", "Junior Associate", "Classroom Teacher"], // APS 3-5
        vec!["Senior", "PhD Candidate", "Associate", "Snr Teacher"],    // APS 5-8
        vec!["Office Manager", "Post-Doc Researcher", "Leading Teacher"], // EL 1 8-10
        vec!["Director", "Senior Lecturer", "Deputy Principal"],        // EL 2 10-13
        vec!["CEO", "Dean", "Partner", "Principal"],                    // SES
    ];

    // Get user input for their job title
    let mut job_title = String::new();
    println!("Enter your job title:");
    io::stdin()
        .read_line(&mut job_title)
        .expect("Failed to read input");
    let job_title = job_title.trim();

    // Get user input for years of experience
    let mut experience = String::new();
    println!("Enter your years of experience:");
    io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read input");
    let experience: u32 = experience.trim().parse().expect("Invalid number");

    // Determine the APS level
    let mut found = false;
    for (i, role_group) in roles.iter().enumerate() {
        if role_group.contains(&job_title) {
            let aps = aps_levels[i];

            // Output based on experience
            println!(
                "Your job title '{}' maps to '{}' with {} years of experience.",
                job_title, aps, experience
            );
            found = true;
            break;
        }
    }

    if !found {
        println!("Job title '{}' not found in the system.", job_title);
    }
}