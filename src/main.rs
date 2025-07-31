use colored::Colorize;
use std::io::{self, Write};

use crate::{
    errors::err::{Error, message},
    options::options::{Opt, args_options},
};

mod config;
mod errors;
mod gpg;
mod options;

fn main() {
    let mut paperpass_args: Vec<String> = std::env::args().collect();
    paperpass_args.remove(0);

    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");

    let mut check_option_double = false;
    match paperpass_args.len() {
        1 => {
            for a in paperpass_args {
                match a {
                    arg if arg == "init" => args_options(Opt::Init),
                    arg if arg == "-list" => {
                        args_options(Opt::ListRecepients(String::from("some text")))
                    }
                    arg if arg == "-h" => args_options(Opt::Help),
                    arg if arg == "-v" => args_options(Opt::Version),
                    _ => {
                        check_option_double = true;
                    }
                }
            }
        }
        3 => {
            if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-s".to_string())
            {
                args_options(Opt::InitParams(
                    paperpass_args[index_a + 1].to_owned(),
                    "".to_owned(),
                    "".to_owned(),
                ));
            } else if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-c".to_string())
            {
                args_options(Opt::InitParams(
                    "".to_owned(),
                    paperpass_args[index_b + 1].to_owned(),
                    "".to_owned(),
                ));
            } else if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-pk".to_string())
            {
                args_options(Opt::InitParams(
                    "".to_owned(),
                    "".to_owned(),
                    paperpass_args[index_c + 1].to_owned(),
                ));
            } else {
                check_option_double = true
            }
        }
        5 => {
            if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-s".to_string())
                && paperpass_args.contains(&"-c".to_string())
            {
                args_options(Opt::InitParams(
                    paperpass_args[index_a + 1].to_owned(),
                    paperpass_args[index_b + 1].to_owned(),
                    "".to_owned(),
                ));
            } else if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-s".to_string())
                && paperpass_args.contains(&"-pk".to_string())
            {
                args_options(Opt::InitParams(
                    paperpass_args[index_a + 1].to_owned(),
                    "".to_owned(),
                    paperpass_args[index_c + 1].to_owned(),
                ));
            } else if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-c".to_string())
                && paperpass_args.contains(&"-pk".to_string())
            {
                args_options(Opt::InitParams(
                    "".to_owned(),
                    paperpass_args[index_b + 1].to_owned(),
                    paperpass_args[index_c + 1].to_owned(),
                ));
            } else {
                check_option_double = true
            }
        }
        7 => {
            if paperpass_args.contains(&"init".to_string())
                && paperpass_args.contains(&"-s".to_string())
                && paperpass_args.contains(&"-c".to_string())
                && paperpass_args.contains(&"-pk".to_string())
            {
                args_options(Opt::InitParams(
                    paperpass_args[index_a + 1].to_owned(),
                    paperpass_args[index_b + 1].to_owned(),
                    paperpass_args[index_c + 1].to_owned(),
                ));
            } else {
                check_option_double = true
            }
        }
        _ => check_option_double = true,
    }

    if check_option_double {
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn catch_stdin() -> String {
    let mut input = String::new();

    let _ = io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect(message(Error::CatchStdin).as_str());

    input.trim().to_string()
}

fn get_index(a: Vec<String>, b: &str) -> usize {
    let mut index = 0;
    if a.contains(&b.to_string()) {
        index = a.iter().position(|mark| mark == &b).unwrap();
    }
    index
}
