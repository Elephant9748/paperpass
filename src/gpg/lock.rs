use crate::utils::binaries::bin_in_box;
use colored::Colorize;
use std::{
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct PaperCrypt<'p> {
    pub storepath: &'p str,
    pub plaintext: &'p str,
    pub uid: &'p str,
    pub paramspath: &'p str,
}

impl<'p> PaperCrypt<'p> {
    pub fn new(storepath: &'p str, plaintext: &'p str, uid: &'p str, paramspath: &'p str) -> Self {
        Self {
            storepath,
            plaintext,
            uid,
            paramspath,
        }
    }
    pub fn newpure(storepath: &'p str, plaintext: &'p str, uid: &'p str) -> Self {
        Self {
            storepath,
            plaintext,
            uid,
            paramspath: "",
        }
    }

    pub fn encrypt_with_params_pure(&mut self) -> std::io::Result<()> {
        if let Ok(run_bin) = bin_in_box() {
            // encrypt data
            let mut echo = Command::new(run_bin[2])
                .args([self.plaintext])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("{}", ":: failed to run echo".bright_yellow()));
            echo.wait()
                .expect("--> Failed to wait echo encrypt_with_params()");
            let gpg = Command::new(run_bin[0])
                .args([
                    "-a",
                    "-o",
                    self.storepath,
                    "-u",
                    self.uid,
                    "-r",
                    self.uid,
                    "--sign",
                    "--encrypt",
                    "--batch",
                    "--yes",
                ])
                .stdin(Stdio::from(echo.stdout.unwrap()))
                .stdout(Stdio::piped())
                .output()
                .unwrap_or_else(|_| panic!("{}", ":: failed to run gpg".bright_yellow()));

            gpg.stderr.is_empty();
        }
        Ok(())
    }
    pub fn encrypt_with_params(&mut self) -> Option<bool> {
        if let Ok(run_bin) = bin_in_box() {
            // path output *.asc
            // #[allow(unused_assignments)]
            // let mut output = String::from("");
            // if file_path.contains("home") || file_path.contains(".asc") {
            //     // for migrate external path
            //     let path_out = saved_path.to_owned();
            //     let mut file_name = String::from("");
            //     if let Some(x) = file_path.rfind('/') {
            //         let y = &file_path[x + 1..];
            //         file_name = y.to_string()
            //     }
            //
            //     output = path_out + "/" + file_name.as_str();
            //     println!("{} Saved to {}", "::".bright_blue(), output);
            // } else {
            //     let path_out = saved_path.to_owned() + "/" + file_path;
            //
            //     // force create dir & filename
            //     output = create_filename(&path_out.to_owned());
            //     println!("{} Saved to {}", "::".bright_blue(), output);
            // }

            let path_out = self.storepath.to_owned() + "/" + self.paramspath;

            // force create dir & filename
            let output = create_filename(&path_out.to_owned());
            println!("{} Saved to {}", "::".bright_blue(), output);

            // encrypt data
            let mut echo = Command::new(run_bin[2])
                .args([self.plaintext])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("{}", ":: failed to run echo".bright_yellow()));
            echo.wait()
                .expect("--> Failed to wait echo encrypt_with_params()");
            let gpg = Command::new(run_bin[0])
                .args([
                    "-a",
                    "-o",
                    output.as_str(),
                    "-u",
                    self.uid,
                    "-r",
                    self.uid,
                    "--sign",
                    "--encrypt",
                ])
                .stdin(Stdio::from(echo.stdout.unwrap()))
                .stdout(Stdio::piped())
                .output()
                .unwrap_or_else(|_| panic!("{}", ":: failed to run gpg".bright_yellow()));

            Some(gpg.stderr.is_empty())
        } else {
            None
        }
    }
}

fn create_filename(path: &str) -> String {
    // get the name, example: "your/path/file"
    let get_name = path.split("/");
    let mut get_name_vec: Vec<&str> = get_name.collect();
    if get_name_vec.last().unwrap().is_empty() {
        get_name_vec.pop();
    }

    // put them back into full of file path
    let mut filename = String::from("");
    let mut i = 0;
    while i < get_name_vec.len() {
        if i == (get_name_vec.len() - 1) {
            filename.push_str(get_name_vec[i]);
        } else {
            filename.push_str(get_name_vec[i]);
            filename.push('/');
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
            create_dir.push('/');
        }

        x += 1;
    }

    if !Path::new(&create_dir).exists() {
        std::fs::create_dir_all(&create_dir).expect(":: force_create_path() for config failed");
    }
}
