use std::{collections::HashMap, env, fs, io, path::PathBuf};

use colored::Colorize;
use crossterm::style::Stylize;

use crate::{
    errors::err::{Error, message},
    utils::{manage_env::ENV_CONFIG, read_config_file},
};

struct Ls {
    store_path: String,
}

impl Ls {
    fn new(store_path: String) -> Self {
        Self {
            store_path: if store_path.is_empty() {
                "".to_string()
            } else {
                store_path
            },
        }
    }
    fn get_store_path(&mut self) {
        let env_config_path = env::var(ENV_CONFIG).expect(message(Error::EnvNotFound).as_str());
        let read_config_from_file = read_config_file(&env_config_path).unwrap();
        self.store_path = read_config_from_file.store.path;
    }
    fn get_to_dirs(&mut self, dir: &mut Dirs, path: PathBuf) -> io::Result<()> {
        if path.is_dir() {
            let paths = fs::read_dir(&path)?;
            for path_result in paths {
                let path = path_result?.path();
                if path.is_dir() {
                    // adding sub_dir
                    let dir_name = path.file_name().unwrap().to_string_lossy().into_owned();
                    let mut subdir = Dirs::new(dir_name.as_str());
                    let _ = self.get_to_dirs(&mut subdir, path);
                    dir.subdir.insert(dir_name, subdir);
                } else {
                    dir.add_file(path.file_name().unwrap().to_str().unwrap());
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Dirs {
    name: String,
    file: Vec<String>,
    subdir: HashMap<String, Dirs>,
}

#[allow(dead_code)]
impl Dirs {
    fn new(name: &str) -> Self {
        Dirs {
            name: name.to_string(),
            file: Vec::new(),
            subdir: HashMap::new(),
        }
    }

    fn add_file(&mut self, file: &str) {
        self.file.push(file.to_string());
    }
    fn print_in_trees(&mut self, indent: usize, dept: usize) {
        for f in &self.file {
            println!(
                "{}{}{}",
                " ".repeat(indent + dept),
                "└── ".bright_yellow(),
                f.to_string().green()
            )
        }

        for (key, value) in &mut self.subdir {
            println!(
                "{}{}{}{}",
                " ".repeat(indent + dept),
                "└── ".bright_yellow(),
                key.bright_cyan(),
                "/".bright_cyan()
            );
            value.print_in_trees(indent + dept, dept);
        }
    }
}

pub fn list_dir_with_params(params: &str) {
    let mut ls = Ls::new("".to_string());
    ls.get_store_path();
    let full_path = ls.store_path.to_owned() + "/" + params.into();

    let mut dirs = Dirs::new(params.into());
    ls.get_to_dirs(&mut dirs, full_path.into()).unwrap();

    // println!("{:#?}", dirs);
    println!("{}", dirs.name.bright_red());
    dirs.print_in_trees(0, 4);
}
