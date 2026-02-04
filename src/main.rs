
use std::io;

// fn main() {
//     println!("What is your name?");

//     let mut name = String::new();

//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line");

//     println!("How old are you?");

//     let mut age = String::new();

//     io::stdin()
//         .read_line(&mut age)
//         .expect("Failed to read line");

//     let mut favorite_color = String::new();

//     println!("What is your favorite color?");

//     io::stdin()
//         .read_line(&mut favorite_color)
//         .expect("Failed to read line");

//     println!("Hello, {}!", name.trim());
//     println!("You are {} years old!", age.trim());
//     println!("Your favorite color is {}!", favorite_color.trim())
// }
fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(& mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please enter a number!")
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    a / b
}

fn modulo(a: f64, b: f64) -> f64 {
    a % b
}

fn main() {
    println!("++++SIMPLE CALCULATOR++++");

    let number1 = get_number("Enter first number:");
    let number2 = get_number("Enter second number");

    println!("\nResults");
    println!("{} + {} = {}", number1, number2, add(number1, number2));
    println!("{} - {} = {}", number1, number2, subtract(number1, number2));
    println!("{} * {} = {}", number1, number2, multiply(number1, number2));
    println!("{} / {} = {}", number1, number2, division(number1, number2));
    println!("{} % {} = {}", number1, number2, modulo(number1, number2));
}