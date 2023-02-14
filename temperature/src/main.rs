//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "A temperature converter")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Alison")]
    Farenheit {},
    Celsius {},
    Kelvin {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Farenheit {}) => {
            let temp = temperature::ask_for_temperature_farenheit();
            let (celsius, kelvin) = temperature::fahrenheit_to_celsius_and_kelvin(temp);
            println!("{}", celsius);
            println!("{}", kelvin);
        }
        Some(Commands::Celsius {}) => {
            let temp = temperature::ask_for_temperature_celsius();
            let (farenheit, kelvin) = temperature::celsius_to_fahrenheit_and_kelvin(temp);
            println!("{}", farenheit);
            println!("{}", kelvin);
        }

        Some(Commands::Kelvin {}) => {
            let temp = temperature::ask_for_temperature_kelvin();
            let (farenheit, celsius) = temperature::kelvin_to_fahrenheit_and_celsius(temp);
            println!("{}", farenheit);
            println!("{}", celsius);
        }
        None => println!("No subcommand was used"),
    }
}
