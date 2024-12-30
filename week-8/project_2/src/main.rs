use std::io;
use std::io::Write;

fn main() {
    let mut name: Vec<String> = Vec::new();
    let mut matric: Vec<String> = Vec::new();
    let mut depar: Vec<String> = Vec::new();
    let mut level: Vec<i32> = Vec::new();

    let mut file = std::fs::File::create("sibas.txt").unwrap();

    file.write_all("PAU SMIS\n".as_bytes()).unwrap();
    let headers = format!("{:<40}{:<40}{:<40}{:<40}\n", "Name", "Matric", "Department", "Level");
    file.write_all(headers.as_bytes()).unwrap();
    loop{

    let mut input1 = String::new();
    println!("Enter your name or type done when you are done");
    io::stdin().read_line(&mut input1).unwrap();
    let name1: String = input1.trim().to_string();
    if &name1 == "done"{
        break;
    }
    name.push(name1.clone());

    
    let mut input2 = String::new();
    println!("Enter your matric no");
    io::stdin().read_line(&mut input2).unwrap();
    let mat: String = input2.trim().to_string();
    matric.push(mat);

    let mut input3 = String::new();
    println!("Enter your department");
    io::stdin().read_line(&mut input3).unwrap();
    let department: String = input3.trim().to_string();
    depar.push(department);

    let mut input4 = String::new();
    println!("Enter your level");
    io::stdin().read_line(&mut input4).unwrap();
    let lev: i32 = input4.trim().parse().unwrap();
    level.push(lev);

  
    }
    let max_length = name.len().max(matric.len()).max(depar.len()).max(level.len());

    for i in 0..max_length {
        let col1 = name.get(i).unwrap();
        let col2 = matric.get(i).unwrap();
        let col3 = depar.get(i).unwrap();
        let col4 = level.get(i).unwrap();
        let row = format!("{:<40}{:<40}{:<40}{:<40}\n", col1, col2, col3, col4);
        file.write_all(row.as_bytes()).unwrap();
    }
}