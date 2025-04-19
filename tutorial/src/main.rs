fn main() {
    println!("Hello, peter");
    another_function(42,'a');
    let (sum1, sum2) = sum(5, 10);
    println!("The sum is: {sum1}");
    println!("The difference is: {sum2}");
    println!("sum = {}, diff = {}", sum1, sum2);
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
fn sum(num1: i32,num2: i32) -> (i32,i32) {
    (num1 + num2, num1 - num2) // This is a tuple
}

