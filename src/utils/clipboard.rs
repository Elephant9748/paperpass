use std::{
    env,
    process::{Command, Stdio},
    thread,
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

pub struct Clip<'a> {
    pub session_type: &'a str,
    pub bin: &'a str,
}

impl<'a> Clip<'a> {
    pub fn new(session: &'a str) -> Self {
        Self {
            session_type: session,
            bin: "",
        }
    }

    pub fn get_binaries(&mut self) -> Option<&'a str> {
        if self.session_type == "wayland" {
            self.bin = "wl-copy";
            Some("wl-copy")
        } else {
            None
        }
    }

    pub fn copy(&self, plaintext: &'a str) {
        Command::new(self.bin)
            .args(&[plaintext])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect(format!("{} {}", message(Error::CopyClipFailed), self.bin).as_str());
    }

    pub fn clear_clipboard(&mut self, timeout: i32) {
        if let Some(clip) = self.get_binaries() {
            let clear_clipboard_duration = timeout;
            let clip_bin = clip.to_owned();
            let thread_clip = thread::spawn(move || {
                Command::new("sh")
                    .args(&[
                        "-c",
                        format!("sleep {} && {} -c", clear_clipboard_duration, clip_bin).as_str(),
                    ])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Thread failed No bash found.");
            });

            if thread_clip.join().is_ok() {
                print!(
                    "{}{}{}",
                    "::".bright_blue(),
                    " Clipboard clear after".bright_yellow(),
                    format!(" {} sec", clear_clipboard_duration).bright_green()
                );
            }
        }
    }
}

pub fn clipboard_copy(params: &str, timeout: i32) {
    let session = env::var(SESSION).unwrap();
    let mut clipboard = Clip::new(session.as_str());
    if clipboard.get_binaries().is_none() {
        println!(
            "No binaries availble to copy to clipboard in sessions type: {}.",
            session
        )
    }

    // get full path
    let configpath = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
    let config = read_config_file(&configpath).unwrap();
    let filename = read_full_filename(params, &config.store.path);
    let plaintext = decrypt_with_params(&filename);

    // copy only the first line
    let plaintext_vec: Vec<&str> = plaintext.split("\n").collect();
    clipboard.copy(plaintext_vec[0]);

    // clear clipboard specific time duration, default timeout is 30 in fn init_options_4
    clipboard.clear_clipboard(timeout);
}
