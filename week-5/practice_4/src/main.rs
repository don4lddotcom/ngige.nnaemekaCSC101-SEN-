fn main() {

    let fulllname = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan- Atlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is: {}", fulllname);
    //check length

    println!("The length my fullname is: {}", fulllname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);
}