fn main() {
    println!("Hello, Peter");


// Basic Control Flow
    

    let number = 5;

    if number < 3 {
        println!("The number is less than 5");
    } else if number == 7 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }

    // if statement
    let no = 4;
    
    if no < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }


    // if let
    let conditinal = true;

    let number = if conditinal { 6 } else { 4 };
    println!("The value of number is: {}", number);


    
    let num = 7;

    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
        //This expression is executed when the condition is false
        if num > 10 {               
            println!("{} is greater than 10", num);
        } else {
            println!("{} is less than or equal to 10", num);
        }
    }



    let a = 10;
    let b = 5;
    let c = 20;

    if a > b && a < c {
        println!("a is greater than b and less than c");
    } else if a < b || a > c {
        println!("a is either less than b or greater than c");
    } else {
        println!("a is neither greater than b nor less than c");
    }



    // Match Statement in enum

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    //define a coin variable
    let my_coin = Coin::Dime;
    let my_coin2 = Coin::Nickel;
    let my_coin3 = Coin::Quarter;
    let my_coin4 = Coin::Penny;

    //Print the value of the coin
    println!("The value of my_coin is: {} cents", value_in_cents(my_coin));
    println!("The value of my_coin2 is: {} cents", value_in_cents(my_coin2));
    println!("The value of my_coin3 is: {} cents", value_in_cents(my_coin3));
    println!("The value of my_coin4 is: {} cents", value_in_cents(my_coin4));
    

    //Loops
    loop {
        println!("This loop will run forever");
        break; // This will exit the loop
    }

    
    



}
 