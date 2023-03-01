//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "Case converter")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Alison")]
    Convert { case: String },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Convert { case }) => {
            let text = case_convert::read_file();
            if case == "snake" {
                case_convert::convert_to_snake_case(text);
            }
            else if case == "title" {
                case_convert::convert_to_title_case(text);
            }
            else if case == "camel" {
                case_convert::convert_to_camel_case(text);
            }
            else if case == "train" {
                case_convert::convert_to_train_case(text);
            }
            else if case == "kebab" {
                case_convert::convert_to_kebab_case(text);
            }
            println!("Done");
        }
        None => println!("No subcommand was used"),
    }
}
