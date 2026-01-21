use std::{collections::BTreeMap, env, fs, io, path::PathBuf};

use colored::Colorize;
use crossterm::style::Stylize;

use crate::{
    errors::err::{Error, message},
    utils::{manage_env::ENV_CONFIG, read_config_file},
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ls {
    pub store_path: String,
}

impl Ls {
    pub fn new(store_path: String) -> Self {
        Self { store_path }
    }
    pub fn get_store_path(&mut self) {
        let env_config_path =
            env::var(ENV_CONFIG).unwrap_or_else(|_| panic!("{}", message(Error::EnvNotFound)));
        let read_config_from_file = read_config_file(&env_config_path).unwrap();
        self.store_path = read_config_from_file.store.path;
    }
    #[allow(clippy::only_used_in_recursion)]
    pub fn get_to_dirs(&self, dir: &mut Dirs, path: PathBuf) -> io::Result<()> {
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
pub struct Dirs {
    pub name: String,
    pub file: Vec<String>,
    pub subdir: BTreeMap<String, Dirs>,
}

#[allow(dead_code)]
impl Dirs {
    pub fn new(name: &str) -> Self {
        Dirs {
            name: name.to_string(),
            file: Vec::new(),
            subdir: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, file: &str) {
        self.file.push(file.to_string());
    }
    // 0 4
    // 4 4
    // 8 4
    // 12 4
    pub fn print_in_trees(&mut self, indent: usize, dept: usize, last_dir: usize) {
        for (i, f) in self.file.iter().enumerate() {
            if last_dir < 1 {
                match i {
                    x if x == self.file.len() - 1 => {
                        print!("{}", " ".repeat((indent + dept) - dept));
                        print!("{}", "│".bright_yellow());
                        print!("{}", " ".repeat(dept));
                        print!("{}", "└──".bright_yellow());
                        println!(" {}", f.to_string().green());
                    }
                    _ => {
                        print!("{}", " ".repeat((indent + dept) - dept));
                        print!("{}", "│".bright_yellow());
                        print!("{}", " ".repeat(dept));
                        print!("{}", "├──".bright_yellow());
                        println!(" {}", f.to_string().green());
                    }
                }
            } else {
                match i {
                    x if x == self.file.len() - 1 => {
                        print!("{}", " ".repeat(indent + dept + 1));
                        print!("{}", "└──".bright_yellow());
                        println!(" {}", f.to_string().green());
                    }
                    _ => {
                        print!("{}", " ".repeat(indent + dept + 1));
                        print!("{}", "├──".bright_yellow());
                        println!(" {}", f.to_string().green());
                    }
                }
            }
        }

        let y = self.subdir.len();
        for (i, (key, value)) in self.subdir.iter_mut().enumerate() {
            match i {
                x if x == y - 1 => {
                    print!("{}", " ".repeat(indent + dept));
                    print!("{}", "└──".bright_yellow());
                    print!(" {}", key.bright_cyan());
                    println!("{}", "/".bright_cyan());
                    value.print_in_trees(indent + dept, dept, 1);
                }
                _ => {
                    print!("{}", " ".repeat(indent + dept));
                    print!("{}", "├──".bright_yellow());
                    print!(" {}", key.bright_cyan());
                    println!("{}", "/".bright_cyan());
                    value.print_in_trees(indent + dept, dept, 0);
                }
            }
        }
    }
    pub fn flattern_dirs(&mut self, subdir_name: &str) -> Vec<String> {
        let mut d = Vec::new();
        for v in &self.file {
            if subdir_name.is_empty() {
                d.push(v.to_owned());
            } else {
                d.push(format!("{}/{}", subdir_name, v));
            }
        }
        for (key, value) in &mut self.subdir {
            #[allow(unused)]
            let mut new_subdir_name = String::from("");
            if subdir_name.is_empty() {
                new_subdir_name = key.to_owned();
            } else {
                new_subdir_name = format!("{}/{}", subdir_name, key);
            };
            d.extend(value.flattern_dirs(new_subdir_name.as_str()));
        }
        d
    }
}

pub fn list_dir_with_params(params: &str) {
    let mut ls = Ls::new("".to_string());
    ls.get_store_path();
    let full_path = ls.store_path.to_owned() + "/" + params;

    let mut dirs = Dirs::new(params);
    ls.get_to_dirs(&mut dirs, full_path.into()).unwrap();

    println!("{}", dirs.name.bright_red());
    dirs.print_in_trees(0, 4, 1);
}

pub fn list_dir_root() {
    let mut ls = Ls::new("".to_string());
    ls.get_store_path();
    let full_path = ls.store_path.to_owned();

    let mut dirs = Dirs::new(full_path.as_str());
    ls.get_to_dirs(&mut dirs, full_path.into()).unwrap();

    // println!("{:#?}", dirs);
    println!("{}", "paperpass".bright_red());
    dirs.print_in_trees(0, 4, 0);
}
