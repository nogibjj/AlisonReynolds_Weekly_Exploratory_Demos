use std::io;

// function to ask for a temperature in fahrenheit
pub fn ask_for_temperature_farenheit() -> f64 {
    // create a new string
    let mut input = String::new();
    // ask for a temperature
    println!("Please enter a temperature in Fahrenheit: ");
    // read the temperature from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // convert the string to a float
    let temp: f64 = input.trim().parse().expect("Please type a number!");
    // return the temperature
    temp
}

// function to ask the user for a temperature in kelvin
pub fn ask_for_temperature_kelvin() -> f64 {
    // create a new string
    let mut input = String::new();
    // ask for a temperature
    println!("Please enter a temperature in Kelvin: ");
    // read the temperature from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // convert the string to a float
    let temp: f64 = input.trim().parse().expect("Please type a number!");
    // return the temperature
    temp
}

// function to ask for a temperature in celsius
pub fn ask_for_temperature_celsius() -> f64 {
    // create a new string
    let mut input = String::new();
    // ask for a temperature
    println!("Please enter a temperature in Celsius: ");
    // read the temperature from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // convert the string to a float
    let temp: f64 = input.trim().parse().expect("Please type a number!");
    // return the temperature
    temp
}

// function to convert fahrenheit to celsius and kelvin
pub fn fahrenheit_to_celsius_and_kelvin(temp: f64) -> (f64, f64) {
    // convert the temperature
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    let kelvin = celsius + 273.15;
    // return the temperature
    (celsius, kelvin)
}

// function to convert celsius to fahrenheit and kelvin
pub fn celsius_to_fahrenheit_and_kelvin(temp: f64) -> (f64, f64) {
    // convert the temperature
    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    let kelvin = temp + 273.15;
    // return the temperature
    (fahrenheit, kelvin)
}

// function to convert kelvin to fahrenheit and celsius
pub fn kelvin_to_fahrenheit_and_celsius(temp: f64) -> (f64, f64) {
    // convert the temperature
    let celsius = temp - 273.15;
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    // return the temperature
    (fahrenheit, celsius)
}
