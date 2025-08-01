use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use colored::Colorize;

use crate::{
    config::Configs,
    errors::err::{Error, message},
};

pub fn insert_with_params(params: &str) {
    println!("{}", params.bright_red());
    let configpath = env::var("PAPERPASS_CONFIG").expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath);
    println!("{:#?}", config);
}

fn read_config_file(filepath: &str) -> Result<Configs, String> {
    let file = File::open(filepath).expect(message(Error::FileNotFound).as_str());
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect(message(Error::CantBufRead).as_str());
    let config_toml: Configs = toml::from_str(&contents).unwrap();
    if contents.is_empty() {
        return Err(message(Error::ResultError).to_string());
    }
    Ok(config_toml)
}
