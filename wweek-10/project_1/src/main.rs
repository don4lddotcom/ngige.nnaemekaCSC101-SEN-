// Define a struct for Laptop
struct Laptop {
    brand: String,
    price: u32,
}

// Implement methods for Laptop
impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define laptops with their respective prices
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Quantity the customer is purchasing from each brand
    let quantity = 3;

    // Calculate total cost
    let total_hp = hp.total_cost(quantity);
    let total_ibm = ibm.total_cost(quantity);
    let total_toshiba = toshiba.total_cost(quantity);
    let total_dell = dell.total_cost(quantity);
    let total_cost = total_hp + total_ibm + total_toshiba + total_dell;

    // Print results
    println!("Cost of {} HP laptops: ₦{}", quantity, total_hp);
    println!("Cost of {} IBM laptops: ₦{}", quantity, total_ibm);
    println!("Cost of {} Toshiba laptops: ₦{}", quantity, total_toshiba);
    println!("Cost of {} Dell laptops: ₦{}", quantity, total_dell);
    println!("Total cost for all laptops: ₦{}", total_cost);
}
