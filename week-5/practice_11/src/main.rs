fn main() {
    let a: i32 = 2; // Binary: 10
    let b: i32 = 3; // Binary: 11

    let mut result: i32;

    // Bitwise AND
    result = a & b;
    println!("(a & b) => {}", result);

    // Bitwise OR
    result = a | b;
    println!("(a | b) => {}", result);

    // Bitwise XOR
    result = a ^ b;
    println!("(a ^ b) => {}", result);

    // Bitwise NOT (1's Complement)
    result = !b;
    println!("(!b) => {}", result);

    // Left Shift
    result = a << b;
    println!("(a << b) => {}", result);

    // Right Shift
    result = a >> b;
    println!("(a >> b) => {}", result);
}
