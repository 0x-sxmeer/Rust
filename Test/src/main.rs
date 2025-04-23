fn main() {
    println!("Hello Sam");
    another_function(42, 'a');   

    // Function with Return Value
    let x = sum(5, 7);
    println!("sum = {}, diff = {}", x.0, x.1);
    println!("The sum and difference are: {:?}", x);

}

// Functions
// parameters and arguments
// -Parameters are variables in the function definition.
// -Arguments are the actual values passed to the function when it is called.
fn another_function(num: i32, letter: char) {
    println!("The value of num is: {}",num);
    println!("The value of letter is: {}",letter);

    // Statements and Expressions
    // -Statements are instructions that perform some action and do not return a value.
    // -Expressions evaluate to a value and do not include a semicolon at the end.
    let x = 5;
    println!("The value of x is: {}",x); 

    let y = {
        let x = 3; // This is a statement, so it needs a semicolon
        x + 1 // This is an expression, so it doesn't need a semicolon
    };
    println!("The value of x is: {}",y);
    println!("Function with Return Value ->");

}


// Function with Return Value
fn sum(num1: i32, num2: i32) -> (i32, i32) {
    // Early return
    return(num1 + num2, num1 - num2); // This is a tuple





}
 