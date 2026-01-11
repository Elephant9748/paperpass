use std::env;

use crate::{
    errors::err::{Error, message},
    utils::{manage_env::ENV_CONFIG, read_config_file},
};
use colored::Colorize;

pub fn show_config() {
    let configpath =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound).as_str()));
    let config = read_config_file(&configpath).unwrap();
    println!();
    println!("Config{}\t", ":".bright_yellow());
    println!(
        "\t{} {}",
        "path".bright_black(),
        config.config.path.bright_yellow()
    );
    println!(
        "\t{} {}",
        "git".bright_black(),
        if config.config.git {
            config.config.git.to_string().bright_green()
        } else {
            config.config.git.to_string().bright_red()
        }
    );
    println!(
        "\t{} {}",
        "key".bright_black(),
        config.gpg.key.bright_cyan()
    );
    println!(
        "\t{} {}",
        "store".bright_black(),
        config.store.path.bright_green()
    );
    println!();

    // println!("\x1b[95m{:#?}", config);
}
