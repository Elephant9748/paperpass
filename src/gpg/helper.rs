use colored::Colorize;
use std::{
    path::Path,
    process::{Command, Stdio},
};

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
    let path_gpg = "/usr/bin/gpg";
    let path_awk = "/usr/bin/awk";
    let mut run_bin = Box::new(Vec::new());

    if Path::new(path_gpg).exists() && Path::new(path_awk).exists() {
        run_bin.push("gpg");
        run_bin.push("awk");
    } else {
        return Err(format!(
            "{}",
            ":: binaries gpg or awk doesnt exists ..!".bright_red()
        ));
    }

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
