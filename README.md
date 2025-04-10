# Rust-
Shadownig


    // Integer
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    
    //let smallest: i8 = whose range is `-128..=127;
    //let lagest: i128 = -170141183460469231731687303715884105728..=170141183460469231731687303715884105727;
    //let samllest: u8 = whose range is 0..=255;
    //let largest: u128 = 340282366920938463463374607431768211455;






    //Number Literals in Rust
    //Rust allows you to express numbers in various formats for readability and convenience. Below are examples of how number literals work:

    //Decimal
    //Example: 98_222

    //Explanation: This is a standard decimal number (base 10). The underscore (_) is used as a visual separator for readability and has no effect on the value.
    //For instance:98_222 is equivalent to 98222.

    //Hexadecimal
    //Example: 0xff

    //Explanation: Hexadecimal numbers (base 16) are prefixed with 0x. Each digit can range from 0-9 and a-f (or A-F for uppercase). For example:

    //0xff represents the decimal value 
    //255
    //255.

    //Octal
    //Example: 0o77

    //Explanation: Octal numbers (base 8) are prefixed with 0o. Each digit ranges from 0-7. For example:

    //0o77 represents the decimal value 
    //63 
    //63.

    //Binary
    //Example: 0b1111_0000

    //Explanation: Binary numbers (base 2) are prefixed with 0b. Each digit is either 0 or 1. Underscores can be used for readability. For example:

    //0b1111_0000 represents the decimal value 
    //240
    //240.

    //Byte Literal (u8 only)
    //Example: b'A'

    //Explanation: Byte literals represent a single byte (8 bits) and are prefixed with b. They are typically used for ASCII characters. For example:

    //b'A' represents the ASCII value of the character 'A', which is 
    //65
    //65 in decimal.


    
**// Floating point numbers
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

