use std::env;
use std::path::PathBuf;

mod commands;
mod lib;

use lib::{handle_race_command, CliContext, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let context = CliContext {
        working_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        config: Config {
            race_endpoint: "127.0.0.1:8080".to_string(),
        },
    };

    match handle_race_command(args, &context) {
        Ok(response) => {
            println!("Status: {}", response.status);
            println!("Message: {}", response.message);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
