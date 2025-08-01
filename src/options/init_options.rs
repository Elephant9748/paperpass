use crate::{
    errors::err::{Error, message},
    options::options::{Opt, args_options},
};
use colored::Colorize;

pub fn init_options_3(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
    if paperpass_args.contains(&"init".to_string()) && paperpass_args.contains(&"-s".to_string()) {
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
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_5(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
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
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_7(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
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
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

fn get_index(a: Vec<String>, b: &str) -> usize {
    let mut index = 0;
    if a.contains(&b.to_string()) {
        index = a.iter().position(|mark| mark == &b).unwrap();
    }
    index
}
