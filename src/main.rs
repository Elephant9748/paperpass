use colored::Colorize;
use std::io::{self, Write};

use crate::{
    errors::err::{Error, message},
    options::{
        init_options::{init_options_2, init_options_3, init_options_5, init_options_7},
        options::{Opt, args_options},
    },
};

mod config;
mod errors;
mod gpg;
mod options;
mod utils;

fn main() {
    let mut paperpass_args: Vec<String> = std::env::args().collect();
    paperpass_args.remove(0);

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
        2 => init_options_2(paperpass_args),
        3 => init_options_3(paperpass_args),
        5 => init_options_5(paperpass_args),
        7 => init_options_7(paperpass_args),
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
