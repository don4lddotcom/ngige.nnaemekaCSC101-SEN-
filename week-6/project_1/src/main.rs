use std::io;

fn main() {
    loop {
        println!("Select a calculation to perform:");
        println!("1: Area of Trapezium");
        println!("2: Area of Rhombus");
        println!("3: Area of Parallelogram");
        println!("4: Area of Cube");
        println!("5: Volume of Cylinder");
        println!("6: Exit");

        let choice = read_input("Enter your choice (1-6):");

        match choice.as_str() {
            "1" => {
                let height: f64 = read_input("Enter the height:").parse().unwrap();
                let base1: f64 = read_input("Enter base1:").parse().unwrap();
                let base2: f64 = read_input("Enter base2:").parse().unwrap();
                let area = area_of_trapezium(height, base1, base2);
                println!("Area of Trapezium: {:.2}", area);
            }
            "2" => {
                let diagonal1: f64 = read_input("Enter diagonal1:").parse().unwrap();
                let diagonal2: f64 = read_input("Enter diagonal2:").parse().unwrap();
                let area = area_of_rhombus(diagonal1, diagonal2);
                println!("Area of Rhombus: {:.2}", area);
            }
            "3" => {
                let base: f64 = read_input("Enter the base:").parse().unwrap();
                let altitude: f64 = read_input("Enter the altitude:").parse().unwrap();
                let area = area_of_parallelogram(base, altitude);
                println!("Area of Parallelogram: {:.2}", area);
            }
            "4" => {
                let side: f64 = read_input("Enter the length of the side:").parse().unwrap();
                let area = area_of_cube(side);
                println!("Area of Cube: {:.2}", area);
            }
            "5" => {
                let radius: f64 = read_input("Enter the radius:").parse().unwrap();
                let height: f64 = read_input("Enter the height:").parse().unwrap();
                let volume = volume_of_cylinder(radius, height);
                println!("Volume of Cylinder: {:.2}", volume);
            }
            "6" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Function to calculate the area of a trapezium
fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

// Function to calculate the area of a rhombus
fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// Function to calculate the area of a parallelogram
fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// Function to calculate the area of a cube
fn area_of_cube(side: f64) -> f64 {
    6.0 * side * side
}

// Function to calculate the volume of a cylinder
fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

// Helper function to read input from the user
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
