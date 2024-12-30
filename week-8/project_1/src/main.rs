use std::io::Write;

fn main() {
    let one = vec!["Lager  ","33 Export ", "Desperados ","Goldberg ","Gulder ","Heineken ","Star "];
    let two = vec!["Stout   ","Legend","Turbo King","Williams"];
    let three = vec!["Non-Alcoholic   ", "Maltina","Amstel Malta","Malta Gold","Fayrouz"];
   

    let mut file = std::fs::File::create("store.txt").unwrap();
    let headers = format!("{:<15}{:<15}{:<15}\n", one[0], two[0], three[0]);
    file.write_all(headers.as_bytes()).unwrap();

    let max_len = one.len().max(two.len()).max(three.len()); // Find the maximum number of rows
    for i in 1..max_len {
        let col1 = one.get(i).unwrap_or(&""); // Get the value or empty string if index out of bounds
        let col2 = two.get(i).unwrap_or(&"");
        let col3 = three.get(i).unwrap_or(&"");
        let row = format!("{:<15}{:<15}{:<15}\n", col1, col2, col3); // Format row with padding
        file.write_all(row.as_bytes()).unwrap();
    }
}