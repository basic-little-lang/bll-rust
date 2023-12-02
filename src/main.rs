use std::{env, process};
use bll_rust::args::Args;
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
    println!("File name: {}", args.file_name());
}
