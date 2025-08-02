use std::{env, path::Path};

use colored::Colorize;

use crate::{
    errors::err::{Error, message},
    gpg::unlock::decrypt_with_params,
    utils::{manage_env::ENV_CONFIG, read_config_file},
};

pub fn show_with_params(params: &str) {
    let configpath = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath).unwrap();
    let filename = read_full_filename(params, &config.store.path);

    if Path::new(&filename).exists() {
        let decrypt = decrypt_with_params(&filename);
        println!("{}", decrypt)
    } else {
        println!(
            "{}{}",
            "File doesnt exists: ".bright_red(),
            filename.italic()
        );
    }
}

pub fn read_full_filename(path: &str, dir_saved: &str) -> String {
    // get the name, example: "your/path/file"
    let get_name = path.split("/");
    let mut get_name_vec: Vec<&str> = get_name.collect();
    if let Some(last) = get_name_vec.last() {
        if last.is_empty() {
            get_name_vec.pop();
        }
    }

    // put them back into full of file path
    let mut filename = String::from("");
    let mut i = 0;
    while i < get_name_vec.len() {
        if i == (get_name_vec.len() - 1) {
            filename.push_str(get_name_vec[i]);
        } else {
            filename.push_str(get_name_vec[i]);
            filename.push_str("/");
        }
        i += 1;
    }

    dir_saved.to_owned() + "/" + &filename + ".asc"
}
