fn main() {
    println!("Hello, world!");
    run_10_debug();

    fn run_10_debug() {
        let base: i32 = 2;
        let limit: i32 = 10;
        println!("Multiplication table for {}:", base);
        for i in 1..=limit {
            let result = base * i;
            println!("{} * {} = {}", base, i, result);
        }
    }
}