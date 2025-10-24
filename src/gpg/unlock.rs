use std::process::{Command, Stdio};

use colored::Colorize;

use crate::utils::binaries::bin_in_box;

pub fn decrypt_with_params(file_path: &str) -> String {
    let run_bin = bin_in_box().unwrap();
    //decrypt data
    let gpg = Command::new(run_bin[0])
        .args(["-d", file_path])
        .stdout(Stdio::piped())
        .output()
        .unwrap_or_else(|_| panic!("{}", ":: failed to run gpg".bright_yellow()));

    String::from_utf8(gpg.stdout).unwrap().trim().into()
}
