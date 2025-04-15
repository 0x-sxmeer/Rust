fn main() {
    println!("Hello, peter");


    //Variables

    let x = 5;
    println!("The value of x is: {x}");


    //Mutable Variables
    let mut y = 5;
    println!("The value of mut y is: {y}");

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
    {
        // Scalar Types
        // - Integer
        // - Floating Point Numbers
        // - Boolean
        // - Character

        //Numeric Operations
        // - Addition
        // - Subtraction
        // - Multiplication
        // - Division
        // - Remainder

        // Compound Types
        // - Tuple
        // - Array

        // Custom Types
        // - Structs
        // - Enums

    }

    // Scalar Types
    let a: i32 = 5;
    let b: f64 = 5.0;
    let c: char = 'a';
    let d: bool = true;
    let e: &str = "Hello, world!";
    println!("A is an integer: {a}");
    println!("B is a float: {b}");
    println!("C is a char: {c}");
    println!("D is a boolean: {d}");
    println!("E is a string: {e}");

    //Numeric Operations
    let x: i32 = 5;
    let y: i32 = 11;
    println!("x = {x},y = {y}");
    //Addition
    let sum = x + y;
    println!("sum = {sum}");
    //Subtraction
    let difference = x - y;
    println!("difference = {difference}");
    //Multiplication
    let product = x * y;
    println!("product = {product}");
    //Division
    let quotient = y / x;
    println!("quotient = {quotient}");
    //Remainder
    let remainder = y % x;
    println!("remainder = {remainder}");
    // Note: In Rust, the division of two integers results in an integer.
    // Floating point division
    let x: f64 = 5.0;
    let y: f64 = 2.0;
    let quotient = x / y;
    println!("quotient = {quotient}");


    //Compound Types

    //Tuple
    let tup: (i32, f64, char) = (500, 6.4, '1');
    println!("The value of tup is: {:?}", tup);

    //-Destructuring tuple
    let (x, y, z) = tup;
    println!("x = {},y = {},z = {}", x, y, z);

    //=Accessing tuple elements by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred = {},six_point_four = {},one = {}", five_hundred, six_point_four, one);


    //Array
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    //-Array with same value
    let b = [3; 5];
    println!("The value of a is: {:?}", b);

    //-Accessing array elements by index
    let first = a[0];
    let second = a[1];
    println!("first = {},second = {}", first, second);

    //-Iterating over array elements
    for i in a.iter() {
        println!("{}", i);
    }

    //-Array length
    let length = a.len();
    println!("length of array a is {}", length);

    //-Array slice
    let slice = &a[0..3];
    println!("slice of array a is {:?}", slice);

    










}