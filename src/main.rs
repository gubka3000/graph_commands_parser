use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use graph_commands_parser::*;

use pest::Parser;

// to make command-line info into struct
#[derive(ClapParser)]
#[command(name = "graph-parser", bin_name = "graph-parser", about = "Graph commands parser")]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

// List of possible commands
// "-- help" command includes by default
#[derive(Subcommand)]
enum Commands {
    Credits,
    Read {
        file: String,
    },
}

// cargo run -- credits 
// cargo run -- help 
// cargo run -- read input_pass_test.txt
// cargo run -- read input_fail_test.txt

fn main() {
    let cli = Args::parse();

    match cli.cmd {

        Commands::Credits => {
            println!("This parser was written by Illia3000");
            println!("on Rust NAUKMA course, for the purpose of learning");
            println!("Date: 20-11-2025")
        }

        Commands::Read { file } => {

            match fs::read_to_string(&file) {

                Ok(content) => {
                    match Command::parse(Rule::file, &content) {
                        Ok(content) => {
                            println!("File was parsed: {}", content);
                        }
                        Err(e) => {
                            println!("{:#?}", e); 
                        }
                    }
                }
                Err(e) => {
                    println!("Cannot read file: {}", e);
                }
            }
 
        }

    }
}


