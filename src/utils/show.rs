use std::{env, path::Path};

use colored::Colorize;

use crate::{
    errors::err::{Error, message},
    gpg::unlock::decrypt_with_params,
    utils::{manage_env::ENV_CONFIG, read_config_file},
};

pub fn show_with_params(params: &str) {
    let configpath =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound)));
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

// 0 -> full_path
// 1 -> path ls
pub fn show_with_params_noprint(params: &str, full: i16) -> String {
    let mut decrypt = String::new();
    decrypt.push_str("");
    match full {
        1 => {
            let configpath =
                env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound)));
            let config = read_config_file(&configpath).unwrap();
            let filename = read_full_filename(params, &config.store.path);

            if Path::new(&filename).exists() {
                decrypt = decrypt_with_params(&filename);
            } else {
                panic!(
                    "{}{}",
                    "File doesnt exists: ".bright_red(),
                    filename.italic()
                );
            }
            decrypt
        }
        0 => {
            if Path::new(&params).exists() {
                decrypt = decrypt_with_params(params);
            } else {
                panic!("{}{}", "File doesnt exists: ".bright_red(), params.italic());
            }
            decrypt
        }
        _ => "".to_string(),
    }
}

pub fn read_full_filename(path: &str, dir_saved: &str) -> String {
    // get the name, example: "your/path/file"
    let get_name = path.split("/");
    let mut get_name_vec: Vec<&str> = get_name.collect();
    if get_name_vec.last().unwrap().is_empty() {
        get_name_vec.pop();
    }

    // put them back into full of file path
    let mut filename = String::from("");
    let mut i = 0;
    while i < get_name_vec.len() {
        if i == (get_name_vec.len() - 1) {
            filename.push_str(get_name_vec[i]);
        } else {
            filename.push_str(get_name_vec[i]);
            filename.push('/');
        }
        i += 1;
    }

    dir_saved.to_owned() + "/" + &filename + ".asc"
}
