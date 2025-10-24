use colored::Colorize;
use std::io::{self, Write};

use crate::{
    errors::err::{Error, message},
    options::{
        init_options::{
            init_options_2, init_options_3, init_options_4, init_options_5, init_options_6,
            init_options_8,
        },
        opt::{Opt, args_options},
    },
    utils::check_session_type,
};

mod config;
mod errors;
mod gpg;
mod options;
mod utils;

fn main() {
    // note for none wayland session
    check_session_type();
    let mut paperpass_args: Vec<String> = std::env::args().collect();
    paperpass_args.remove(0);

    let mut check_option_double = false;
    match paperpass_args.len() {
        1 => {
            for a in paperpass_args.iter() {
                match a {
                    arg if arg == "init" => args_options(Opt::Init),
                    arg if arg == "ls" => args_options(Opt::ListDirRoot),
                    arg if arg == "-config" || arg == "--config" => args_options(Opt::ShowConfig),
                    arg if arg == "-lk" => {
                        args_options(Opt::ListRecepients(String::from("some text")))
                    }
                    arg if arg == "-h" || arg == "--help" => args_options(Opt::Help),
                    arg if arg == "-v" || arg == "--version" => args_options(Opt::Version),
                    _ => {
                        check_option_double = true;
                    }
                }
            }
        }
        2 => init_options_2(paperpass_args.to_owned()),
        3 => init_options_3(paperpass_args.to_owned()),
        4 => init_options_4(paperpass_args.to_owned()),
        5 => init_options_5(paperpass_args.to_owned()),
        6 => init_options_6(paperpass_args.to_owned()),
        8 => init_options_8(paperpass_args.to_owned()),
        _ => check_option_double = true,
    }

    if check_option_double {
        let mut not_menu = String::new();
        for val in &paperpass_args {
            not_menu.push_str(val.as_str());
            not_menu.push(' ');
        }
        println!(
            "{}{}{}",
            not_menu.bright_red(),
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn catch_stdin() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| panic!("{}", message(Error::CatchStdin)));

    input.trim().to_string()
}
