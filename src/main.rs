use std::{env, process, fs};
use bll_rust::{args::Args, parser::{tokens::tokenize_str, ParsedTokens}};
use colored::Colorize;

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = match Args::build(&args) {
        Some(args) => args,
        None => {
            eprintln!("{}: Cannot parse args", "error".bold().red());
            eprintln!("{}: <filename>", "usage".bold());
            process::exit(1);
        }
    };
    
    let contents = match fs::read_to_string(args.file_name()) {
        Ok(str) => str,
        Err(err) => {
            eprintln!("{}: {}", "error".bold().red(), err.to_string());
            process::exit(1);
        }
    };

    let tokens = match tokenize_str(&contents) {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{}: {}", "error".bold().red(), err);
            process::exit(1);
        }
    };

    let tokens = match ParsedTokens::convert(&tokens) {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{}: {}", "error".bold().red(), err);
            process::exit(1);
        }
    };

    println!("{:?}", tokens);

}
