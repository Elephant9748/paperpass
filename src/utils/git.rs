use std::process::{Command, Stdio};

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
