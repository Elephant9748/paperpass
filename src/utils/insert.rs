use std::env;

use colored::Colorize;

use crate::{
    catch_stdin,
    errors::err::{Error, message},
    gpg::lock::encrypt_with_params,
    utils::{git::git_commit, manage_env::ENV_CONFIG, read_config_file, valid_store_path},
};

pub fn insert_with_params(params: &str) {
    let configpath =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound)));
    let config = read_config_file(&configpath).unwrap();
    let path_to_saved = valid_store_path(config.store.path.as_str());
    let params_to_saved = valid_store_path(params);

    // prompt what to store
    print!("Enter password for {}: ", params);
    let pass = catch_stdin();

    // encrypt pass
    let go_encrypt = encrypt_with_params(
        path_to_saved.as_str(),
        pass.as_str(),
        config.gpg.key.as_str(),
        params_to_saved.as_str(),
    );

    if go_encrypt {
        println!("{}{}", "::".bright_blue(), " Encrypt Ok.".green())
    } else {
        println!("{}{}", "::".bright_blue(), " Encrypt Failed.".red())
    }

    //git commit
    if config.config.git {
        git_commit(config.store.path.as_str(), params_to_saved);
    } else {
        println!("{}{}", "::".bright_blue(), " Git commit false.".red());
    }
}

#[allow(dead_code)]
pub fn insert_for_migration(params: &str, secrets: &str, path_to_saved: &str, key_name: &str) {
    let configpath =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound)));
    let config = read_config_file(&configpath).unwrap();
    let params_to_saved = valid_store_path(params);

    // encrypt pass
    let go_encrypt =
        encrypt_with_params(path_to_saved, secrets, key_name, params_to_saved.as_str());

    if go_encrypt {
        println!("{}{}", "::".bright_blue(), " Encrypt Ok.".green())
    } else {
        eprintln!("{}{}", "::".bright_blue(), " Encrypt Failed.".red())
    }

    //git commit
    if config.config.git {
        git_commit(config.store.path.as_str(), params_to_saved);
    } else {
        println!("{}{}", "::".bright_blue(), " Git commit false.".red());
    }
}
