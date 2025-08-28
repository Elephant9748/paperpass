use colored::Colorize;
use std::process::{Command, Stdio};

use crate::utils::binaries::bin_in_box;

#[derive(Debug)]
pub struct GpgHelper {
    keys: Option<Vec<String>>,
}

impl GpgHelper {
    pub fn new(key: Vec<String>) -> Self {
        Self { keys: Some(key) }
    }
    pub fn get_all(&self) -> Option<Vec<String>> {
        self.keys.to_owned()
    }
    #[allow(dead_code)]
    pub fn get_by_name(&self, n: &str) -> Option<String> {
        if let Some(a) = self.keys.clone().unwrap().into_iter().find(|f| f == n) {
            Some(a)
        } else {
            None
        }
    }
}

// gnupg private keys by uid
pub fn listprivatekeys() -> Result<Vec<String>, String> {
    let run_bin = bin_in_box().unwrap();

    let gpg = Command::new(run_bin[0])
        .args(&["--list-secret-keys", "--with-colons"])
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("{}", ":: failed to run gpg".bright_yellow()).as_str());

    let awk = Command::new(run_bin[1])
        .args(&["-F", ":", "$1 == \"uid\" {print $10}"])
        .stdin(Stdio::from(gpg.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("{}", ":: failed to run awk".bright_yellow()).as_str());

    let awk = Command::new(run_bin[1])
        .args(&["{print $1}"])
        .stdin(Stdio::from(awk.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("{}", ":: failed to run awk".bright_yellow()).as_str());

    let out = String::from_utf8_lossy(&awk.wait_with_output().unwrap().stdout).into_owned();
    let stdoutvec: Vec<String> = out
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|b| b.into())
        .collect();

    Ok(stdoutvec)
}
