use std::io;


fn main() {
    let mut user_name_input = String::new();
    println!("Please enter your name: ");
    io::stdin().read_line(&mut user_name_input).expect("Please enter only your name!!");
    let name = user_name_input.trim();
    let mut incentive:f32 = 0.0;

    let mut age_input = String::new();
    println!("Please type in your age: ");
    io::stdin().read_line(&mut age_input).expect("Please type in your age in numbers!");
    let age:i32 = age_input.trim().parse().expect("INVALID INPUT:\nPlease type in only numbers!");

    let mut user_experience_input = String::new();
    println!("Please type true if you are experienced and false if not experienced: ");
    io::stdin().read_line(&mut user_experience_input).expect("Please type in your experience!!");
    let binding = user_experience_input.to_lowercase();
    let is_experienced = binding.trim();


    
    if is_experienced == "true" && age >= 40{
        incentive = 1_560_000.0;
        if name.ends_with("s"){
            println!("{}' incentive is {} per year.", name, incentive);
        }else{
        println!("{}'s incentives is {} per year.", name, incentive);
        }
    }
    else if is_experienced == "true" && 30 <= age && age< 40{
        incentive = 1_480_000.0;
        if name.ends_with('s'){
            println!("{}' incentive is {} per year.", name, incentive);
        }else{
        println!("{}'s incentive is {} per year.", name, incentive);
        }
    }

    else if is_experienced == "true" && age < 28{
        incentive = 1_300_000.0;
        if name.ends_with('s'){
            println!("{}' incentive is {} per month.", name, incentive);
        }else{
        println!("{}'s incentive is {} per month.", name, incentive);
        }
    }
    else if is_experienced == "false"{
        incentive = 100_000.0;
        if name.ends_with('s'){
            println!("{}' incentive is {} per year.", name, incentive);
        }else{
        println!("{}'s incentive is {} per year.", name, incentive);
        }
    }
    else {
        println!("Invalid input. Please type true or false for your experience.");
    }
}