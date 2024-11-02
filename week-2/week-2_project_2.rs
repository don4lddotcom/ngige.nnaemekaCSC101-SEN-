fn main() {
  // Definiing the amount and quantities of each item
    let amount1 = 450000.0;
    let qty1 = 2;
    
    let amount2 = 1500000.0;
    let qty2 = 1;
    
    let amount3 = 750000.0;
    let qty3 = 3;
    
    let amount4 = 2850000.0;
    let qty4 = 3;
    
    let amount5 = 250000.0;
    let qty5 = 1;

    // Finding the total sum

    let total_sum = (amount1 * qty1 as f64)
                  + (amount2 * qty2 as f64)
                  + (amount3 * qty3 as f64)
                  + (amount4 * qty4 as f64)
                  + (amount5 * qty5 as f64);

    // Finding the average

    let total_items = qty1 + qty2 + qty3 + qty4 + qty5;
    let average = total_sum / total_items as f64;

    println!("The total sum of the sales amounts is: N{:.2}", total_sum);
    println!("The average of the sales amounts is: N{:.2}", average);
} 
