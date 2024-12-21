fn main() {
    // initialization of tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}", datatype_tuple);

    // initialization of tuple without data type
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_datatype_tuple);

    // accessing elements of tuple at index 0
    println!("First element of tuple = {}", datatype_tuple.0);

    // accessing elements of tuple at index 1
    println!("Second element of tuple = {}", datatype_tuple.1);

    // accessing elements of tuple at index 2
    println!("Third element of tuple = {}", datatype_tuple.2);
}
