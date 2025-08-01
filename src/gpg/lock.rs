use std::{
    path::Path,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::utils::binaries::bin_in_box;

pub fn encrypt_with_params(saved_path: &str, plaintext: &str, uid: &str, file_path: &str) -> bool {
    let run_bin = bin_in_box().unwrap();
    println!("{:#?}", run_bin);

    // path output *.asc
    let path_out = saved_path.to_owned() + file_path;

    // force create dir & filename
    let output = create_filename(&path_out.to_owned());
    println!("{}", output);

    // encrypt data
    let echo = Command::new(run_bin[2])
        .args(&[plaintext])
        .stdout(Stdio::piped())
        .spawn()
        .expect(format!("{}", ":: failed to run echo".bright_yellow()).as_str());
    let gpg = Command::new(run_bin[0])
        .args(&[
            "-a",
            "-o",
            output.as_str(),
            "-u",
            uid,
            "-r",
            uid,
            "--sign",
            "--encrypt",
        ])
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .stdout(Stdio::piped())
        .output()
        .expect(format!("{}", ":: failed to run awk".bright_yellow()).as_str());

    if gpg.stderr.is_empty() { true } else { false }
}

fn create_filename(path: &str) -> String {
    // get the name, example: "your/path/file"
    let get_name = path.split("/");
    let mut get_name_vec: Vec<&str> = get_name.collect();
    if let Some(last) = get_name_vec.last() {
        if last.is_empty() {
            get_name_vec.pop();
        }
    }

    // put them back into full of file path
    let mut filename = String::from("");
    let mut i = 0;
    while i < get_name_vec.len() {
        if i == (get_name_vec.len() - 1) {
            filename.push_str(get_name_vec[i]);
        } else {
            filename.push_str(get_name_vec[i]);
            filename.push_str("/");
        }
        i += 1;
    }

    force_create_dir(get_name_vec);

    filename + ".asc"
}

fn force_create_dir(path_in_vec: Vec<&str>) {
    let mut create_dir = String::from("");
    let mut x = 0;
    while x < (path_in_vec.len() - 1) {
        if x == (path_in_vec.len() - 2) {
            create_dir.push_str(path_in_vec[x]);
        } else {
            create_dir.push_str(path_in_vec[x]);
            create_dir.push_str("/");
        }

        x += 1;
    }

    if !Path::new(&create_dir).exists() {
        std::fs::create_dir_all(&create_dir).expect(":: force_create_path() for config failed");
    }
}
