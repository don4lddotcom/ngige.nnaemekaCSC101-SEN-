fn main() {
    // Array with explicit integer datatype
    let arr1: [i32; 4] = [10, 20, 30, 40];
    println!("\nArray with explicit integer datatype");
    println!("Array is: {:?}", arr1);
    println!("Array size is: {}", arr1.len());

    // Array with implicit float datatype
    let arr2 = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nArray with implicit float datatype");
    println!("Array is: {:?}", arr2);
    println!("Array size is: {}", arr2.len());

    // Array with default values
    let arr3: [i32; 8] = [-1; 8];
    println!("\nArray with default values");
    println!("Array is: {:?}", arr3);
    println!("Array size is: {}", arr3.len());
}
