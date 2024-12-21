fn main() {
    // Declare a tuple
    let b: (i32, bool, f64) = (30, true, 4.9);

    // Call the print function
    print(b);
}

fn print(x: (i32, bool, f64)) {
    println!("Inside print method");

    // Destructure the tuple into distinct variables
    let (age, is_male, cgpa) = x;

    // Print the values
    println!("Age is {}, isMale? {}, GPA is {}", age, is_male, cgpa);
}
