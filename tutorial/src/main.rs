fn main() {
    println!("Hello, peter");


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

   let t = "hello world";
    print!("valve of t is {}", t);


}
