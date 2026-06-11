pub mod binaries;
pub mod clipboard;
pub mod delete;
pub mod edit;
pub mod fromcsv;
pub mod genpass;
pub mod git;
pub mod insert;
pub mod ls;
pub mod manage_env;
pub mod migrate;
pub mod show;
pub mod totp;

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use colored::Colorize;

use crate::{config::Configs, errors::err::PaperpassError, utils::manage_env::SESSION};

pub fn read_config_file(filepath: &str) -> Result<Configs, String> {
    let file = File::open(filepath).unwrap_or_else(|_| panic!("{}", PaperpassError::FileNotFound));
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("{}", PaperpassError::CantBufRead));
    let config_toml: Configs = toml::from_str(&contents).unwrap();
    if contents.is_empty() {
        return Err(PaperpassError::ResultConfig.to_string());
    }
    Ok(config_toml)
}

pub fn valid_store_path(path: &str) -> String {
    let mut store_path = path.to_string();
    if !path.contains("/") {
        store_path = path.to_owned() + "/";
    }
    store_path
}

pub fn check_session_type() {
    let session = env::var(SESSION).unwrap_or_else(|_| panic!("{}", PaperpassError::EnvNotFound));
    if session != "wayland" {
        println!("{}{}", "Your current session: ".yellow(), session.red(),);
        println!(
            "{}{}",
            "Some function doesnt work properly under none ".yellow(),
            "wayland session.".green()
        );
    }
}
