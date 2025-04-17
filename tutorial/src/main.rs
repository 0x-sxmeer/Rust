fn main() {
    println!("Hello, peter");
    another_function(42,'a');
}



//Parameters and Arguments
fn another_function(num: i32,letter: char) {
    println!("The value of num is: {num}");
    println!("The value of letter is: {letter}");

//Statements and Expressions
    // -Statements are instructions that perform some action and do not return a value.
    // -Expressions evaluate to a value and do not include a semicolon at the end.

    let x = 5; // This is a statement, so it needs a semicolon
    let y = {
        let x = 3;
        x + 1 // This is an expression, so it doesn't need a semicolon
    };
    println!("The value of y is: {y}");
    println!("The value of x is: {x}");


}


//Function with Return Value
fn five() -> i32 {
    5 // This is an expression, so it doesn't need a semicolon
}

