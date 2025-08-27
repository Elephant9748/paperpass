use std::{
    env,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::{
    errors::err::{Error, message},
    utils::{
        binaries::bin_in_box, manage_env::ENV_CONFIG, read_config_file, show::read_full_filename,
        valid_store_path,
    },
};

pub fn edit_with_params(params: &str) {
    let configpath = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath).unwrap();
    let path_to_saved = valid_store_path(config.store.path.as_str());
    let params_to_saved = valid_store_path(params);
    let filename = read_full_filename(params, &config.store.path);

    let decrypt_file = env::current_dir().unwrap().to_str().unwrap().to_owned()
        + "/"
        + path_to_saved.to_owned().as_str()
        + params_to_saved.to_owned().as_str();

    let run_bin = bin_in_box().unwrap();

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    // decrypt to file
    let _ = Command::new(run_bin[0])
        .args(&["-o", decrypt_file.as_str(), "-d", filename.as_str()])
        .stdout(Stdio::piped())
        .status()
        .expect(
            format!(
                "{}",
                ":: edit_with_params() decrypt gpg exited".bright_yellow()
            )
            .as_str(),
        );

    // open editor
    let _ = Command::new(editor)
        .arg(&decrypt_file)
        .status()
        .expect(format!("{}", ":: edit_with_params() editor exited".bright_yellow()).as_str());

    // encrypt to file
    let _ = Command::new(run_bin[0])
        .args(&[
            "-a",
            "-o",
            format!("{}.asc", decrypt_file).as_str(),
            "-u",
            &config.gpg.key,
            "-r",
            &config.gpg.key,
            "--sign",
            "--encrypt",
            &decrypt_file,
        ])
        .status()
        .expect(
            format!(
                "{}",
                ":: edit_with_params() encrypt gpg exited".bright_yellow()
            )
            .as_str(),
        );

    // shreding file
    let shreding = Command::new("shred")
        .args(&["-uzn", "100", &decrypt_file])
        .stdout(Stdio::piped())
        .output()
        .expect(format!("{}", ":: edit_with_params() shred exited".bright_yellow()).as_str());
    println!("{}", String::from_utf8_lossy(&shreding.stdout));
}
