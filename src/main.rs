use std::io;

fn main() {
    println!("Welcome to the temperature converter!");
    println!("Please enter a temperature value in Celsius: ");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: usize = value
        .trim()
        .parse()
        .expect("Value entered was not a number");

    let converter = (value * 9/5) + 32;

    println!("Converting {value} degree Celsius to Fahrenheit is {converter}F");

}
