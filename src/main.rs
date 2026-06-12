use std::io::{self, Write};

use crate::{
    errors::err::PaperpassError, options::command_options::Args, utils::check_session_type,
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

    let mut command_args = Args::new(paperpass_args);
    command_args.set_run();
    command_args.run_args();
}

pub fn catch_stdin() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| panic!("{}", PaperpassError::CatchStdin));

    input.trim().to_string()
}
