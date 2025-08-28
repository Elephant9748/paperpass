use std::{env, path::Path};

use colored::Colorize;

use crate::{
    errors::err::{Error, message},
    utils::{manage_env::ENV_CONFIG, read_config_file, valid_store_path},
};

pub fn delete_with_params(params: &str) {
    let configpath = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath).unwrap();
    let path_to_saved = valid_store_path(config.store.path.as_str());
    let params_to_saved = valid_store_path(params);

    // a file
    let full_path_of_file = path_to_saved.to_owned() + "/" + params_to_saved.as_str() + ".asc";
    // a dir
    let full_path_of_dir = path_to_saved.to_owned() + "/" + params_to_saved.as_str();

    if Path::new(&full_path_of_file).is_file() {
        std::fs::remove_file(full_path_of_file).expect("delete_with_params() remove file failed");
        println!("{}{}", "::".bright_blue(), " Delete Ok.")
    } else if Path::new(&full_path_of_dir).is_dir() {
        std::fs::remove_dir_all(full_path_of_dir).expect("delete_with_params() remove dir failed");
        println!("{}{}", "::".bright_blue(), " Delete Ok.")
    } else {
        println!(
            "{}{}\"{}\"{}",
            "::".bright_blue(),
            " Delete Failed. ".bright_red(),
            full_path_of_file.bright_yellow(),
            " Is not a file"
        );
    }
}
