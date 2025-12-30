use crate::{
    errors::err::{Error, message},
    options::opt::{Opt, args_options},
};
use colored::Colorize;

pub fn init_options_2(paperpass_args: Vec<String>) {
    let insert_index = get_index(paperpass_args.to_owned(), "insert");
    let show_index = get_index(paperpass_args.to_owned(), "show");
    let edit_index = get_index(paperpass_args.to_owned(), "edit");
    let delete_index = get_index(paperpass_args.to_owned(), "delete");
    if paperpass_args.contains(&"insert".to_string()) {
        args_options(Opt::InsertParams(
            paperpass_args[insert_index + 1].to_owned(),
        ));
    } else if paperpass_args.contains(&"edit".to_string()) {
        args_options(Opt::EditParams(paperpass_args[edit_index + 1].to_owned()));
    } else if paperpass_args.contains(&"show".to_string()) {
        args_options(Opt::ShowParams(paperpass_args[show_index + 1].to_owned()));
    } else if paperpass_args.contains(&"delete".to_string()) {
        args_options(Opt::DeleteParams(
            paperpass_args[delete_index + 1].to_owned(),
        ));
    } else if paperpass_args.contains(&"-c".to_string()) {
        // default clear clipboard timeout is 30sec
        args_options(Opt::Copy(paperpass_args[show_index + 1].to_owned(), 30));
    } else if paperpass_args.contains(&"totp".to_string()) {
        args_options(Opt::TotpCreate(
            paperpass_args[show_index + 1].to_owned(),
            0,
        ));
    } else if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-git".to_string())
    {
        args_options(Opt::InitParams(
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "y".to_owned(),
        ));
    } else if paperpass_args.contains(&"ls".to_string()) {
        args_options(Opt::ListDir(paperpass_args[show_index + 1].to_owned()));
    } else if paperpass_args.contains(&"migrate".to_string()) {
        args_options(Opt::Migrate(paperpass_args[show_index + 1].to_owned()));
    } else {
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_3(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
    if paperpass_args.contains(&"init".to_string()) && paperpass_args.contains(&"-s".to_string()) {
        args_options(Opt::InitParams(
            paperpass_args[index_a + 1].to_owned(),
            "".to_owned(),
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
            "".to_owned(),
        ));
    } else if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-pk".to_string())
    {
        args_options(Opt::InitParams(
            "".to_owned(),
            "".to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "".to_owned(),
        ));
    } else if paperpass_args.contains(&"totp".to_string())
        && paperpass_args.contains(&"-c".to_string())
    {
        // default clear clipboard timeout is 30sec
        args_options(Opt::TotpCreate(paperpass_args[index_b + 1].to_owned(), 30));
    } else {
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_4(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-c");
    let index_b = get_index(paperpass_args.to_owned(), "-time");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
    if paperpass_args.contains(&"-c".to_string()) && paperpass_args.contains(&"-time".to_string()) {
        args_options(Opt::Copy(
            paperpass_args[index_a + 1].to_owned(),
            paperpass_args[index_b + 1]
                .to_owned()
                .parse::<i32>()
                .unwrap(),
        ));
    } else if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-pk".to_string())
        && paperpass_args.contains(&"-git".to_string())
    {
        args_options(Opt::InitParams(
            "".to_owned(),
            "".to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "y".to_owned(),
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
            "".to_owned(),
        ));
    } else if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-c".to_string())
        && paperpass_args.contains(&"-pk".to_string())
    {
        args_options(Opt::InitParams(
            "".to_owned(),
            paperpass_args[index_b + 1].to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "".to_owned(),
        ));
    } else {
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_6(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-pk".to_string())
        && paperpass_args.contains(&"-s".to_string())
        && paperpass_args.contains(&"-git".to_string())
    {
        args_options(Opt::InitParams(
            "".to_owned(),
            paperpass_args[index_b + 1].to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "y".to_owned(),
        ));
    } else if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-pk".to_string())
        && paperpass_args.contains(&"-c".to_string())
        && paperpass_args.contains(&"-git".to_string())
    {
        args_options(Opt::InitParams(
            paperpass_args[index_a + 1].to_owned(),
            "".to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "y".to_owned(),
        ));
    } else {
        println!(
            "{}{}",
            "::".bright_blue(),
            message(Error::OptionsNotFound).bright_yellow()
        );
    }
}

pub fn init_options_8(paperpass_args: Vec<String>) {
    let index_a = get_index(paperpass_args.to_owned(), "-s");
    let index_b = get_index(paperpass_args.to_owned(), "-c");
    let index_c = get_index(paperpass_args.to_owned(), "-pk");
    if paperpass_args.contains(&"init".to_string())
        && paperpass_args.contains(&"-pk".to_string())
        && paperpass_args.contains(&"-c".to_string())
        && paperpass_args.contains(&"-git".to_string())
    {
        args_options(Opt::InitParams(
            paperpass_args[index_a + 1].to_owned(),
            paperpass_args[index_b + 1].to_owned(),
            paperpass_args[index_c + 1].to_owned(),
            "y".to_owned(),
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
        index = a.iter().position(|mark| mark == b).unwrap();
    }
    index
}
