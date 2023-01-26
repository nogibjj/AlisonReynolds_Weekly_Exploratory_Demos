//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "A coin flipper")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Flip {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Flip {}) => {
            let result = random::coin();
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}