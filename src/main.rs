use std::io;

fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number!")
}

fn get_operation() -> String {
    println!("\nChoose operation:");
    println!("1. Add (+)");
    println!("2. Subtract (-)");
    println!("3. Multiply (*)");
    println!("4. Divide (/)");
    println!("Enter your choice (1-4):");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input.trim().to_string()
}

fn calculate(num1: f64, num2: f64, operation: &str) -> f64 {
    match operation {
        "1" => num1 + num2,
        "2" => num1 - num2,
        "3" => num1 * num2,
        "4" => num1 / num2,
        _ => {
            println!("Invalid operation! Defaulting to addition.");
            num1 + num2
        }
    }
}

fn main() {
    println!("=== Interactive Calculator ===");
    
    let num1 = get_number("Enter first number:");
    let num2 = get_number("Enter second number:");
    let operation = get_operation();
    
    let result = calculate(num1, num2, &operation);
    
    let symbol = match operation.as_str() {
        "1" => "+",
        "2" => "-",
        "3" => "*",
        "4" => "/",
        _ => "?",
    };
    
    println!("\n{} {} {} = {}", num1, symbol, num2, result);
}