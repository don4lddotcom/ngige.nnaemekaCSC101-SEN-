fn main() {
    // An array of numbers
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // Create a slice of 2nd and 3rd elements
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced - {:?}", slice1);

    // Omit the start index
    let slice2 = &numbers[..3];
    // This means the slice starts from index 0 and goes up to index 3 (exclusive)
    println!("Index 0 to index 3 sliced - {:?}", slice2);

    // Omit the end index
    let slice3 = &numbers[2..];
    // This means the slice starts from index 2 and goes up to the end of the array
    println!("Index 2 to the end sliced - {:?}", slice3);

    // Omit the start index and the end index
    let slice4 = &numbers[..];
    // This means the slice includes the entire array
    println!("Entire array sliced - {:?}", slice4);
}
