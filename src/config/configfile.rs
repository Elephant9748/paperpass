use std::{
    env, io,
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::{catch_stdin, utils::git::git_init_run};

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

pub fn set_options_config_path() -> Result<&'static str, String> {
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
        "config"
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "2".bright_green(),
        "]".bright_blue(),
        "~/.config/paperpass/"
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "3".bright_green(),
        "]".bright_blue(),
        ".config"
    );
    print!(
        "\n{}",
        "Where to save config file (default value 1): ".bright_white()
    );
    let input_path = catch_stdin();
    if input_path.is_empty() {
        Ok("")
    } else {
        let mut path = "paperpass.toml";
        if check_valid(Valid::Num(input_path.to_owned())) {
            match input_path.parse::<i32>().unwrap() {
                1 => path = "",
                2 => path = "~/.config/paperpass",
                3 => path = ".config",
                _ => path = "~/.config/paperpass",
            }
        }
        Ok(path)
    }
}

pub fn set_config_path(p: String) -> Result<String, String> {
    if p.starts_with("~") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[2..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path.into())
        }
    } else if p.is_empty() {
        let default_path = "config";
        let path = force_create_dir(default_path.to_string());
        Ok(path.into())
    } else {
        let path = force_create_dir(p);
        Ok(path.into())
    }
}

pub fn set_store_path(p: String) -> Result<String, String> {
    if p.starts_with("~") {
        let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
        let mut full_home_dir = PathBuf::from(home_dir);
        full_home_dir.push(&p[2..]);
        if check_valid(Valid::Path(full_home_dir.display().to_string())) {
            Ok(full_home_dir.display().to_string())
        } else {
            let path = force_create_dir(full_home_dir.display().to_string());
            Ok(path.into())
        }
    } else if p.is_empty() {
        let default_path = "store";
        let path = force_create_dir(default_path.to_string());
        Ok(path.into())
    } else {
        let path = force_create_dir(p);
        Ok(path.into())
    }
}

pub fn set_git(g: String) -> Result<bool, String> {
    if g == "y" || g == "Y" {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn git_init(init: bool, store: String) -> io::Result<()> {
    if init {
        let _ = git_init_run(store.as_str());
    }
    Ok(())
}

fn force_create_dir(b: String) -> String {
    if !Path::new(&b).exists() {
        std::fs::create_dir_all(b.to_owned()).expect(":: force_create_dir(b) failed");
    }
    b
}
