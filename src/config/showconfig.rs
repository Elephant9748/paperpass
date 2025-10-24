use std::env;

use crate::{
    errors::err::{Error, message},
    utils::{manage_env::ENV_CONFIG, read_config_file},
};

pub fn show_config() {
    let configpath =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound).as_str()));
    let config = read_config_file(&configpath).unwrap();
    println!("\x1b[95m{:#?}", config);
}
