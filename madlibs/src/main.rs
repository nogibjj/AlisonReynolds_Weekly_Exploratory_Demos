//A command-line tool to flip a coin
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Alison", about = "A madlib generator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Alison")]
    Create {
        subject: String,
        building: String,
        animal: String,
        verb: String,
        adjective: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Create {
            subject,
            building,
            animal,
            verb,
            adjective,
        }) => {
            let mut madlib = String::new();

            // add the first line to the madlib
            madlib.push_str("It's late at night. You're studying for your ");

            // add the second line to the madlib
            madlib.push_str(subject.as_str());

            // add the third line to the madlib
            madlib.push_str(" exam in  ");

            // add the fourth line to the madlib
            madlib.push_str(building.as_str());

            // add the fifth line to the madlib
            madlib.push_str(" when you hear a ");

            // add the sixth line to the madlib
            madlib.push_str(animal.as_str());

            // add the seventh line to the madlib
            madlib.push_str(". You");

            // add the eighth line to the madlib
            madlib.push_str(verb.as_str());

            // add the ninth line to the madlib
            madlib.push_str(" outside and find your friends laughing ");

            // add the tenth line to the madlib
            madlib.push_str(adjective.as_str());
            println!("{}", madlib);
        }
        None => println!("No subcommand was used"),
    }
}
