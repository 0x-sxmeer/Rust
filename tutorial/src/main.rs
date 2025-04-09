fn main() {
    println!("Hello, peter");

    let mut x = 5;
    println!("valve of x is {}", x);

   let y = 6;
    println!("valve of x is {}", x);
    println!("valve of y is {}", y);

    {
        let x = 10;
        println!("valve of x is {}", x);
    }



    x = 5;
    println!("valve of x is {}", x);
}
