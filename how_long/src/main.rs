//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "Time converter")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Alison")]
    Date {
        date: String,
    },
    Time {
        time: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Date { date }) => {
            let result = how_long::how_long_date(how_long::date_string_to_datetime(date));
            println!("{}", result);
        }
        Some(Commands::Time { time }) => {
            let result = how_long::how_long_time(how_long::time_string_to_naive_time(time));
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
