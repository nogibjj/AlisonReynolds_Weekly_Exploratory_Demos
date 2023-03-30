//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "A classification tool")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Alison")]
    Tree {split: String},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Tree{split}) => {
            let _res = dt_classification::decision_tree(split);
        }
        None => println!("No subcommand was used"),
    }
}
