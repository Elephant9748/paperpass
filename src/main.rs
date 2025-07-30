use colored::Colorize;
use std::io::{self, Write};

use crate::options::options::{Opt, args_options};

mod config;
mod gpg;
mod options;

fn main() {
    let mut paperpass_args: Vec<String> = std::env::args().collect();
    paperpass_args.remove(0);

    for a in paperpass_args {
        match a {
            arg if arg == "init" => args_options(Opt::Init),
            arg if arg == "-list" => args_options(Opt::ListRecepients(String::from("some text"))),
            arg if arg == "-h" => args_options(Opt::Help),
            arg if arg == "-v" => args_options(Opt::Version),
            _ => {
                println!(
                    "{}{}",
                    "::".bright_blue(),
                    " args not found \"-h\" to see available options".bright_yellow()
                );
                println!();
                args_options(Opt::Help);
            }
        }
    }
}

pub fn catch_stdin() -> String {
    let mut input = String::new();

    let _ = io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect(":: catch_stdin failed.");

    input.trim().to_string()
}
