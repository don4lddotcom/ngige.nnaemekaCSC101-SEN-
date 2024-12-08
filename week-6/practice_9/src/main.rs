fn main() {
    let arr = [15, 12, 9, 8, 0]; // Initialize the array
    println!("Array is {:?}", arr); // Print the array
    
    println!("Array size is: {}", arr.len()); // Print the size of the array
    
    for val in arr.iter() { // Iterate over the array
        println!("Value is: {}", val); // Print each value
    }
}
