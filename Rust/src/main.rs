fn main() {
    println!("Hello, world!");
}

// This program prints the multiplication table for 2 up to 10.
fn main() {
    let base = 2;       // The base number for the multiplication table
    let limit = 10;     // The upper limit of the table

    // Print a header for clarity
    println!("Multiplication Table for {}:", base);

    // Loop from 1 to limit (inclusive) to generate the table
    for i in 1..=limit {
        let result = base * i;  // Calculate the product
        println!("{} x {} = {}", base, i, result);  // Print each line
    }
}
