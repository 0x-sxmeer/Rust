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





}
 