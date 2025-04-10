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
  

    // Integer
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Floating point numbers
    {
        let decimal: u128 = 98_222;
        let hex: u128 = 0xef;
        let octal: u128 = 0o67;
        let binary: u128 = 0b1111_0000;
    
        println!("Decimal: {}", decimal); // Output: Decimal: 98222
        println!("Hexadecimal: {}", hex); // Output: Hexadecimal: 255
        println!("Octal: {}", octal);     // Output: Octal: 63
        println!("Binary: {}", binary);   // Output: Binary: 240
    }
    



}