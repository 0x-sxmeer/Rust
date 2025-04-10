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

    
    
    //constants
    const MAX_POINTS: i32 = -100_000;
    println!("valve of max points is {}", MAX_POINTS);
    //We can not change the value of MAX_POINTS
    //MAX_POINTS = 100; //error: cannot assign twice to immutable variable `MAX_POINTS`
    
    
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

    //data types
    let guess: u32 = "42".parse().expect("not a number");
    println!("valve of guess is {}", guess);

    //array
    let a = [1, 2, 3, 4, 5];
    println!("valve of a is {}", a[3]);










} 