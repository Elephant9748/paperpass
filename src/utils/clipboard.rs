use std::{
    env,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::{
    errors::err::{Error, message},
    gpg::unlock::decrypt_with_params,
    utils::{
        manage_env::{ENV_CONFIG, SESSION},
        read_config_file,
        show::read_full_filename,
    },
};

struct Clip<'a> {
    session_type: &'a str,
    bin: &'a str,
}

impl<'a> Clip<'a> {
    fn new(session: &'a str) -> Self {
        Self {
            session_type: session,
            bin: "",
        }
    }

    fn get_binaries(&mut self) -> Option<&'a str> {
        if self.session_type == "wayland" {
            self.bin = "wl-copy";
            Some("wl-copy")
        } else {
            None
        }
    }

    fn copy(&self, plaintext: &'a str) -> bool {
        let copy = Command::new(self.bin)
            .args(&[plaintext])
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Failed to run {}", self.bin).as_str());

        if copy.stderr.is_none() { true } else { false }
    }
}

pub fn clipboard_copy(params: &str) {
    let session = env::var(SESSION).unwrap();
    let mut clipboard = Clip::new(session.as_str());
    if clipboard.get_binaries().is_none() {
        println!(
            "No binaries availble to copy to clipboard in sessions type: {}.",
            session
        )
    }

    let configpath = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath).unwrap();
    let filename = read_full_filename(params, &config.store.path);
    let plaintext = decrypt_with_params(&filename);

    let done = clipboard.copy(plaintext.as_str());
    if !done {
        println!(
            "{}{}",
            "::".bright_blue(),
            "Copy to clipboard failed".bright_red()
        )
    }
}
