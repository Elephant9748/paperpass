use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use colored::Colorize;

use crate::{
    catch_stdin,
    config::Configs,
    errors::err::{Error, message},
    gpg::lock::encrypt_with_params,
};

pub fn insert_with_params(params: &str) {
    let configpath = env::var("PAPERPASS_CONFIG").expect(message(Error::EnvNotFound).as_str());
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
        println!("{}{}", "::".bright_blue(), " Insert Ok.")
    } else {
        println!("{}{}", "::".bright_blue(), " Insert Failed.")
    }
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

fn valid_store_path(path: &str) -> String {
    let mut store_path = path.to_string();
    if !path.contains("/") {
        store_path = path.to_owned() + "/";
    }
    store_path
}
