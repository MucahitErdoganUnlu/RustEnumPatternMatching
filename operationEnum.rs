use std::io;

enum Operation{
    Add(f64 ,f64),
    Subtract(f64 ,f64 ),
    Multiply(f64 ,f64 ),
    Divide(f64 ,f64 ),
}

fn calculate(op: &Operation) -> f64 {
    match op{
        Operation::Add(x,y) => {
            x+y
        }
        Operation::Subtract(x,y) => {
            x-y
        }
        Operation::Multiply(x,y) => {
            x*y
        }
        Operation::Divide(x,y) => {
            x/y
        }
    }
}
fn main() {

    // Prompt the user for the first number
    let mut first_number = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Invalid number");

    // Prompt the user for the operation
    let mut operation = String::new();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();


    // Prompt the user for the second number
    let mut second_number = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Invalid number");

    
    
    // Perform the operation based on user input
    let result = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation. Please enter +, -, *, or /.");
            return;
        }
    };
    println!["Result: {}", calculate(&result)];

}
