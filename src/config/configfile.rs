use std::{
    env,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::catch_stdin;

enum Valid {
    Num(String),
    Path(String),
}

fn check_valid(val: Valid) -> bool {
    match val {
        Valid::Num(a) => a.as_str().chars().all(|c| c.is_numeric()),
        Valid::Path(a) => {
            if Path::new(&a).exists() {
                true
            } else {
                false
            }
        }
    }
}

pub fn set_config_path() -> Result<&'static str, String> {
    println!(
        "\n{}{}",
        "::".bright_blue(),
        " Config Located".bright_yellow()
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "1".bright_green(),
        "]".bright_blue(),
        "paperpass.toml"
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "2".bright_green(),
        "]".bright_blue(),
        "~/config/paperpass/paperpass.toml"
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "3".bright_green(),
        "]".bright_blue(),
        ".config/paperpass/paperpass.toml"
    );
    print!("\n{}", "Where to save config file: ".bright_white());
    let input_path = catch_stdin();
    let mut path = "paperpass.toml";
    if check_valid(Valid::Num(input_path.to_owned())) {
        match input_path.parse::<i32>().unwrap() {
            1 => path = "paperpass.toml",
            2 => path = "~/config/paperpass/paperpass.toml",
            3 => path = ".config/paperpass/paperpass.toml",
            _ => println!("{}", ":: input key not valid [number]."),
        }
    }
    Ok(path)
}

pub fn set_store_path(p: String) -> Result<String, String> {
    if p.starts_with("~") {
        let home_dir = env::var("HOME").expect(":: var $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[2..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path.into())
        } else {
            Err(":: set_store_path() failed".to_string())
        }
    } else if p.is_empty() {
        let default_path = "store/";
        let path = force_create_dir(default_path.to_string());
        Ok(path.into())
    } else {
        let path = force_create_dir(p);
        Ok(path.into())
    }
}

fn force_create_dir(b: String) -> String {
    if !Path::new(&b).exists() {
        std::fs::create_dir(b.to_owned()).expect(":: set_store_path failed create store/");
    }
    b
}
