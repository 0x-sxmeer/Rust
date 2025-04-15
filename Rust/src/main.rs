fn main() {
    println!("Hello, peter");


    //Variables

    let x = 5;
    println!("The value of x is: {x}");


    //Mutable Variables
    let mut y = 5;
    println!("The value of mut y is: {}",y);

    y = 6;
    println!("The value of y is: {y}");


    //Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");


    //Shadowing
    let z = 5;
    let z = z+1;

    {
        let z = z*2;
        println!("The value of z is: {z}"); 
        // it will give the output of 12
    }

    println!("The value of z is: {z}");

    //Shadowing with mutable
    {
        let mut z = z*2;
        println!("The value of z is: {z}");
        z = 10;
        println!("The value of z is: {z}");
    }

    println!("The value of z is: {z}");


    //Data Types
    let a: i32 = 5;
    let b: f64 = 5.0;
    let c: char = 'a';
    let d: bool = true;
    let e: &str = "Hello, world!";
    let f: [i32; 5] = [1, 2, 3, 4, 5];
    let g: (i32, f64, char) = (5, 5.0, 'a');
    println!("The value of a is = {}, b is = {}, c is = {}, d is = {}, e is = {}, f is = {:?}, g is = {:?}", a, b, c, d, e, f, g);








}