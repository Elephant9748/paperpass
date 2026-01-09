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
        Valid::Path(a) => Path::new(&a).exists(),
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
        "~/.config/paperpass/".white()
    );
    println!(
        "   {}{}{} {}",
        "[".bright_blue(),
        "2".bright_green(),
        "]".bright_blue(),
        "~/paperpass".white()
    );
    print!(
        "\n{}",
        "Where to save config file (default value 1): ".bright_white()
    );
    let input_path = catch_stdin();
    if input_path.is_empty() {
        Ok("~/.config/paperpass")
    } else {
        let mut path = "paperpass.toml";
        if check_valid(Valid::Num(input_path.to_owned())) {
            match input_path.parse::<i32>().unwrap() {
                1 => path = "~/.config/paperpass",
                2 => path = "~/paperpass",
                _ => path = "~/.config/paperpass",
            }
        }
        Ok(path)
    }
}

pub fn set_config_path(p: String) -> Result<String, String> {
    match p {
        ref x if x.starts_with("~") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[2..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref x if x.starts_with("HOME") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[6..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref x if x.is_empty() => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut default_path = PathBuf::from(home_dir);
            default_path.push(".config/paperpass");
            let path = force_create_dir(default_path.display().to_string());
            Ok(path)
        }
        _ => {
            let path = force_create_dir(p);
            Ok(path)
        }
    }
}

pub fn set_store_path(p: String) -> Result<String, String> {
    match p {
        ref y if y.starts_with("~") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[2..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref y if y.starts_with("HOME") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[6..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref y if y.is_empty() => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut default_path = PathBuf::from(home_dir);
            default_path.push("paperpass_store");
            let path = force_create_dir(default_path.display().to_string());
            Ok(path)
        }
        _ => {
            let path = force_create_dir(p);
            Ok(path)
        }
    }
}

pub fn home_path(p: String) -> Result<String, String> {
    match p {
        ref z if z.starts_with("~") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[2..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref z if z.starts_with("HOME") => {
            let home_dir = env::var("HOME").expect(":: VAR $HOME doesnt exists");
            let mut full_home_dir = PathBuf::from(home_dir);
            full_home_dir.push(&p[6..]);
            if check_valid(Valid::Path(full_home_dir.display().to_string())) {
                Ok(full_home_dir.display().to_string())
            } else {
                let path = force_create_dir(full_home_dir.display().to_string());
                Ok(path)
            }
        }
        ref z if z.is_empty() => Err("--> Validate ~/, $HOME : PATH is empty".to_string()),
        _ => {
            let path = force_create_dir(p);
            Ok(path)
        }
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
        std::fs::create_dir_all(&b).expect(":: force_create_dir(b) failed");
    }
    b
}
