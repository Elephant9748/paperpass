use std::{
    env,
    io::{self, Write},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use chrono::{DateTime, Utc};
use colored::Colorize;
use crossterm::{
    event::{Event, KeyCode, read},
    terminal,
};
use totp_rs::TOTP;

use crate::{
    errors::err::{Error, message},
    gpg::unlock::decrypt_with_params,
    utils::{
        clipboard::Clip,
        manage_env::{ENV_CONFIG, SESSION},
        read_config_file,
        show::read_full_filename,
    },
};

struct Totp {
    session: String,
    full_path: String,
}

impl Totp {
    fn new(session: String) -> Self {
        Self {
            session: session,
            full_path: "".into(),
        }
    }

    fn decrypt_file(&self) -> String {
        decrypt_with_params(self.full_path.as_str())
    }

    fn get_full_path_of_file(&mut self, params: &str) {
        let env_config_path = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
        let read_config_from_file = read_config_file(&env_config_path).unwrap();
        let full_path_file = read_full_filename(params, &read_config_from_file.store.path);
        if self.session == "wayland" {
            self.full_path = full_path_file;
        } else {
            println!(
                "{}{}{}{}{}",
                "Oops".bright_yellow(),
                " not running wayland ".bright_red(),
                SESSION.bright_yellow(),
                ": ".bright_yellow(),
                self.session.bright_green()
            );
        }
    }
}

pub fn totp_create(params: &str, timeout: i32) {
    let mut totp = Totp::new(env::var(SESSION).unwrap());
    totp.get_full_path_of_file(params);
    let plaintext = totp.decrypt_file();

    // get otp url from plaintext
    let plainvec: Vec<&str> = plaintext.split("\n").collect();
    let totprs = TOTP::from_url(plainvec[2]).expect("Url is empty not allow in TOTP::from_url()");

    //timestamp
    let timestamp: DateTime<Utc> = Utc::now();

    //clipboard copy with clear timeout 30sec
    let sess = env::var(SESSION).unwrap();
    let mut clip = Clip::new(sess.as_str());
    if clip.get_binaries().is_none() {
        println!(
            "No binaries availble to copy to clipboard in sessions type: {}.",
            sess
        )
    }

    if timeout > 0 {
        clip.copy(
            totprs
                .generate(timestamp.timestamp().try_into().unwrap())
                .as_str(),
        );
        clip.clear_clipboard(timeout);
    } else {
        let q_is_pressed = Arc::new(Mutex::new(false));
        let q_is_pressed_clone = Arc::clone(&q_is_pressed);
        thread::spawn(move || {
            loop {
                let event = read().unwrap();
                if event == Event::Key(KeyCode::Char('q').into()) {
                    *q_is_pressed_clone.lock().unwrap() = true;
                    break;
                }
            }
        });

        terminal::enable_raw_mode().expect("Enable crossterm raw mod failed.");
        while !*q_is_pressed.lock().unwrap() {
            let timestamp: DateTime<Utc> = Utc::now();
            let out_otp = totprs.generate(timestamp.timestamp().try_into().unwrap());
            print!(
                "\r{} {}{}{} {}{}{}",
                out_otp.to_string().bright_green(),
                "(".blue(),
                totprs.ttl().unwrap().to_string().red(),
                ")".blue(),
                "(".blue(),
                "Press 'q' to exit or ctrl + c".bright_yellow(),
                ")".blue(),
            );
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
        terminal::disable_raw_mode().expect("Disable crossterm raw mod failed.");
    }
}
