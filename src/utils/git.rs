use std::process::{Command, Stdio};

use chrono::{DateTime, Utc};
use colored::Colorize;

use crate::utils::binaries::bin_in_box;

pub fn git_init_run(store: &str) -> Result<bool, String> {
    let run_bin = bin_in_box().unwrap();

    let git = Command::new(run_bin[3])
        .args(&["init", store])
        .stdout(Stdio::piped())
        .output()
        .expect(format!("{}", ":: Git Init failed".bright_yellow()).as_str());

    if git.stderr.is_empty() {
        Ok(true)
    } else {
        Err(format!(
            "Git Init failed: {}",
            String::from_utf8_lossy(&git.stderr)
        ))
    }
}

pub fn git_commit(store_path: &str, params: String) {
    let run_bin = bin_in_box().unwrap();
    let current_utc: DateTime<Utc> = Utc::now();

    let _ = Command::new(run_bin[3])
        .args(&["-C", store_path, "add", "."])
        .stdout(Stdio::piped())
        .output()
        .expect(format!("{}", ":: git_commit() git add . failed".bright_yellow()).as_str());

    let git_commit = Command::new(run_bin[3])
        .args(&[
            "-C",
            store_path,
            "commit",
            "-m",
            format!("{} {}", params, current_utc.format("%Y-%m-%d_%H_%M_%S")).as_str(),
        ])
        .stdout(Stdio::piped())
        .output()
        .expect(format!("{}", ":: git_commit() git commit -m failed".bright_yellow()).as_str());

    println!(
        "{}",
        String::from_utf8_lossy(&git_commit.stdout).into_owned()
    );
    println!("{}{}", "::".bright_blue(), " Git commit true.");
}
