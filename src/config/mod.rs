pub mod configfile;
pub mod showconfig;

use crate::{
    catch_stdin,
    config::configfile::{
        git_init, set_config_path, set_git, set_options_config_path, set_store_path,
    },
    gpg::helper::{GpgHelper, listprivatekeys},
    options::banner::prompt_banner,
    utils::manage_env::{ENV_CONFIG, set_env},
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub config: Config,
    pub gpg: Gpg,
    pub store: Store,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpg {
    pub key: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub git: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub path: String,
}

pub fn init_config() {
    let recipient = GpgHelper::new(listprivatekeys().unwrap());
    prompt_banner();
    println!("\n{} {}", "::".bright_blue(), "Recipient".yellow());
    for (mut key, val) in recipient.get_all().unwrap().into_iter().enumerate() {
        key += 1;
        println!(
            "   {}{}{} {}",
            "[".bright_purple(),
            key.to_string().bright_green(),
            "]".bright_purple(),
            val
        );
    }
    print!(
        "\n{}",
        "which key to use (default value 1): ".bright_white()
    );
    let input_key = catch_stdin();
    // use default key index
    let input_key = if input_key.is_empty() {
        "1".to_string()
    } else {
        input_key
    };

    let input_config_path = set_options_config_path().unwrap().to_string();

    // force create dir config if doesnt exists
    let check_home_dir = set_config_path(input_config_path.to_owned()).unwrap();
    let forcepath = force_create_dir_on_file(check_home_dir.to_owned().as_str());
    let mut configpath = "paperpass.toml".to_string();
    if forcepath.is_empty() {
        configpath = check_home_dir + "/paperpass.toml";
    };

    // where data is store
    println!("\n{}{}", "::".bright_blue(), " Store data".bright_yellow());
    println!(
        "{}",
        "   example path: ~/[whereyoustoredata] or $HOME/[whereyoustoredata]"
            .italic()
            .bright_magenta()
    );
    print!("\n{}", "where to store data: ".bright_white());
    let input = catch_stdin();
    let store_path = set_store_path(input).unwrap();

    // store include git
    print!("\n{}", "Use git init (default n) (y/n)? ".bright_white());
    let input = catch_stdin();
    let store_git = set_git(input);
    // git init in store
    git_init(store_git.to_owned().unwrap(), store_path.to_owned()).unwrap();

    let config = Configs {
        config: Config {
            path: configpath.to_owned(),
            git: store_git.unwrap(),
        },
        gpg: Gpg {
            key: recipient.get_all().unwrap()[input_key.parse::<usize>().unwrap() - 1].to_string(),
        },
        store: Store { path: store_path },
    };
    let toml = toml::to_string(&config).unwrap();
    let file = File::create(&configpath).expect(":: paperpass.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(toml.as_bytes());
    let _ = buf_writer.flush();

    init_done(config);
}

// init config with params
pub fn init_config_with_params(opt1: &str, opt2: &str, opt3: &str, opt4: &str) {
    let recipient = GpgHelper::new(listprivatekeys().unwrap());

    let opt1 = if opt1.is_empty() { "store" } else { opt1 };
    let opt3 = if opt3.is_empty() {
        &recipient.get_all().unwrap()[0].to_string()
    } else {
        opt3
    };

    // config
    let config_check_home = set_config_path(opt2.to_owned()).unwrap();
    let forcepath = force_create_dir_on_file(&config_check_home);
    let mut configpath = "paperpass.toml".to_string();
    if forcepath.is_empty() {
        configpath = config_check_home + "/paperpass.toml";
    };

    //set env
    set_env(configpath.as_str());

    // data store
    let store_path = set_store_path(opt1.to_string()).unwrap();

    //store with git
    let store_git = set_git(opt4.to_string());
    // git init in store
    git_init(store_git.to_owned().unwrap(), store_path.to_owned()).unwrap();

    let config = Configs {
        config: Config {
            path: configpath.to_owned(),
            git: store_git.unwrap(),
        },
        gpg: Gpg {
            key: opt3.to_string(),
        },
        store: Store { path: store_path },
    };
    let toml = toml::to_string(&config).unwrap();
    let file = File::create(&configpath).expect(":: paperpass.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(toml.as_bytes());
    let _ = buf_writer.flush();

    init_done(config);
}

fn force_create_dir_on_file(path: &str) -> String {
    let mut find_dir = String::from("");
    if !Path::new(&path).exists() {
        find_dir = path.replace("paperpass.toml", "");
        std::fs::create_dir_all(&find_dir).expect(":: force_create_path() for config failed");
        return find_dir;
    }
    find_dir
}

fn init_done(config: Configs) {
    println!(
        "\x1b[36m{:#?}\n{}{}",
        config,
        "::".bright_blue(),
        " Write config file succeed.".bright_green()
    );
    println!(
        "\n{}",
        "Please set your environment variable".bright_green()
    );
    println!("{}", "Bash edit '~/.bashrc' put:".bright_yellow());
    println!("_____________________________________");
    println!("  \"export {}={}\"", ENV_CONFIG, config.config.path);
    println!("_____________________________________");
    println!(
        "{}",
        "fish edit '~/config/config.fish' put:".bright_yellow()
    );
    println!("_____________________________________");
    println!("if status --is-interactive");
    println!("  # ...");
    println!("  # ...");
    println!("  \"set -x {} {}\"", ENV_CONFIG, config.config.path);
    println!("end");
    println!("_____________________________________");
}
