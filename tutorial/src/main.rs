
fn main() {
    println!("Hello, peter");

    //variable shadowing
    let x = 5;
    println!("valve of x is {}", x);

    let y = 6;
    println!("valve of y is {}", y);

    {
        let x = 10;
        println!("valve of x is {}", x);
    }

    let y = x+2;
    println!("valve of new x {}", y);

    let t =5;
    println!("valve of t is {}", t*4);

    //mutable variable
    let mut x = 5;
    println!("valve of x is {}", x);
    x = 6;
    println!("valve of x is {}", x);

    //shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("valve of x is {}", x);

    //constants
    const MAX_POINTS: i32 = -100_000;
    println!("valve of max points is {}", MAX_POINTS);
    //We can not change the value of MAX_POINTS
    //MAX_POINTS = 100; //error: cannot assign twice to immutable variable `MAX_POINTS`
    
    // Data types
    // scalar types
    // -integer
    // -floating point numbers
    // -boolean
    // -character
    // compound types
    // -tuple
    // -array
    // Custom Types
    // -structs
    // -enums

    {
    // Integer
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Floating point numbers

        let decimal: u128 = 98_222;
        let hex: u128 = 0xef;
        let octal: u128 = 0o67;
        let binary: u128 = 0b1111_0000;
    
        println!("Decimal: {}", decimal); // Output: Decimal: 98222
        println!("Hexadecimal: {}", hex); // Output: Hexadecimal: 255
        println!("Octal: {}", octal);     // Output: Octal: 63
        println!("Binary: {}", binary);   // Output: Binary: 240
    }
    

    // Floating point numbers
    
    let x = 2.0; // f64 default
    let y: f32 = 3.0; // f32
    
    println!("x = {},y = {}", x, y);


    //Numeric operations
    let sum = x + y; // Addition
    let difference = x - y; // Subtraction
    let product = x * y; // Multiplication
    let quotient = x / y; // Division
    let remainder = x % y; // Remainder
    // Note: In Rust, the division of two integers results in an integer.
    // For example, 5 / 2 = 2, not 2.5.
    println!("sum = {},difference = {},product = {},quotient = {},remainder = {}", sum, difference, product, quotient, remainder);


    // Boolean type
    let t = true; // implicit declaration
    let f: bool = false; // explicit declaration

    println!("t = {},f = {}", t, f);

    // Boolean operations
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let number = 8;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,2");
    }
    
    let number2 = 8;
    if number2 % number == 0 {
        println!("number2 is divisible by number");
    } else {
        println!("number2 is not divisible by number");
    }
 

    // Character type
    let c = 'z'; // implicit declaration
    let x = 'X';
    println!("c = {},x = {}",c ,x);

    // iterate over characters in a string
    for c in "Hello".chars() {
        println!("{}", c);
    }


    

}