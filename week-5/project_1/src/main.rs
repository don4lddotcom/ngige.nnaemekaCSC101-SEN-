use std::io;

fn main() {
    let p_price: i32 = 3200;
    let f_price: i32 = 3000;
    let a_price: i32 = 2500;
    let e_price: i32 = 2000;
    let w_price: i32 = 2500;

    let mut food_price: i32 = 0;

    println!("Welcome to the Pan-Atlantic University Restaurant!!\n Below is a menu of the available food and prices");
    println!(
        "
    Menu                              Price
    P = Poundo Yam/Edinkaiko Soup      - N3,200
    F = Fried Rice & Chicken           - N3,000
    A = Amala & Ewedu Soup             - N2,500
    E = Eba & Egusi Soup               - N2,000
    W = White Rice & Stew              - N2,500"
    );

    loop {
        let mut food_name_input = String::new();
        println!(
            "Enter the name of the food you want to eat(Let it start with the letter(i.e., P, A, F, E, W) and type cancel when done ordering: "
        );
        io::stdin()
            .read_line(&mut food_name_input)
            .expect("Failed to read input");
        let food_name = food_name_input.trim().to_lowercase();

        if food_name == "cancel" {
            break;
        }
        let mut pre_order: i32;

        if food_name == "p" {
            pre_order = p_price;
        } else if food_name == "f" {
            pre_order = f_price;
        } else if food_name == "a" {
            pre_order = a_price;
        } else if food_name == "e" {
            pre_order = e_price;
        } else if food_name == "w" {
            pre_order = w_price;
        } else {
            println!("Invalid food option. Please try again.");
            continue;
        }

        let mut food_order_input = String::new();
        println!("Please enter the number of each type of food item you want to order (Please take note of the order): ");
        io::stdin()
            .read_line(&mut food_order_input)
            .expect("Please enter only the number of each type of food item you want to order");
        let quantity: i32 = food_order_input
            .trim()
            .parse()
            .expect("INVALID INPUT:\nPlease type in only numbers!");

        food_price += pre_order * quantity;

        println!("Your total food price is N{}", food_price);
    }

    if food_price > 10_000 {
        let discount = (food_price * 5) / 100;
        let updated_price = food_price - discount;
        println!(
            "Since your total purchase was above N10,000, we are thrilled to inform you that we have freely given you a discount of 5%!\nYour updated price is N{}\nThank you for shopping with us! We look forward to seeing you again!!",
            updated_price
        );
    } else if food_price == 0 {
        println!("You haven't made any purchases. Thank you for visiting!");
    } else {
        println!(
            "Your total price for food is N{}.\nThank you for dining with us, and we hope to see you again soon!",
            food_price
        );
    }
}
