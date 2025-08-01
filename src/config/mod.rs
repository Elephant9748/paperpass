pub mod configfile;

use crate::{
    catch_stdin,
    config::configfile::{set_config_path, set_store_path},
    gpg::helper::{GpgHelper, listprivatekeys},
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
    git: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub path: String,
}

pub fn init_config() {
    let recipient = GpgHelper::new(listprivatekeys().unwrap());
    println!("\n{} {}", "::".bright_blue(), "Recipient".yellow());
    for (mut key, val) in recipient.get_all().unwrap().into_iter().enumerate() {
        *&mut key += 1;
        println!(
            "   {}{}{} {}",
            "[".bright_purple(),
            key.to_string().bright_green(),
            "]".bright_purple(),
            val
        );
    }
    print!("\n{}", "which key to use: ".bright_white());
    let input_key = catch_stdin();
    let input_config_path = set_config_path().unwrap().to_string();

    // force create dir config if doesnt exists
    let forcepath = force_create_dir_on_file(&input_config_path.to_owned());

    // where data is store
    println!("\n{}{}", "::".bright_blue(), " Store data".bright_yellow());
    println!(
        "{}",
        "   example path: ~/[whereyoustoredata] or just full path [where/you/store/data]"
            .italic()
            .bright_magenta()
    );
    print!("\n{}", "where to store data: ".bright_white());
    let input = catch_stdin();
    let store_path = set_store_path(input).unwrap();

    let config = Configs {
        config: Config {
            path: input_config_path,
            git: false,
        },
        gpg: Gpg {
            key: format!(
                "{}",
                &recipient.get_all().unwrap()[input_key.parse::<usize>().unwrap() - 1]
            )
            .to_string(),
        },
        store: Store { path: store_path },
    };
    let toml = toml::to_string(&config).unwrap();
    let file =
        File::create(format!("{}paperpass.toml", forcepath)).expect(":: paperpass.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(toml.as_bytes());
    let _ = buf_writer.flush();

    init_done(config);
}

// init config with params
pub fn init_config_with_params(opt1: &str, opt2: &str, opt3: &str) {
    let recipient = GpgHelper::new(listprivatekeys().unwrap());

    let opt1 = if opt1.is_empty() { "store/" } else { opt1 };
    let opt3 = if opt3.is_empty() {
        &recipient.get_all().unwrap()[0].to_string()
    } else {
        opt3
    };

    // config
    let forcepath = force_create_dir_on_file(&opt2.to_owned());
    let mut configpath = "paperpass.toml".to_string();
    if !forcepath.is_empty() {
        configpath = forcepath.as_str().to_owned() + "/paperpass.toml";
    };

    //set env
    set_env(configpath.as_str());

    // data store
    let store_path = set_store_path(opt1.to_string()).unwrap();

    let config = Configs {
        config: Config {
            path: configpath.to_owned(),
            git: false,
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

static ENV_KEY: &str = "PAPERPASS_CONFIG";
fn set_env(path: &str) {
    unsafe {
        std::env::set_var(ENV_KEY, path);
    }
}

fn init_done(config: Configs) {
    println!(
        "{:#?}\n{}{}",
        config,
        "\u{1F600}".bright_blue(),
        " init config succeed.".bright_green()
    );
    println!(
        "\n{}",
        "please set your environment variable".bright_yellow()
    );
    println!("{}", "------------------------------------".bright_yellow());
    println!("{}", "Bash edit '~/.bashrc' put:".bright_yellow());
    println!("_____________________________________");
    println!("  \"export {}={}\"", ENV_KEY, config.config.path);
    println!("_____________________________________");
    println!(
        "{}",
        "fish edit '~/config/config.fish' put:".bright_yellow()
    );
    println!("_____________________________________");
    println!("if status --is-interactive");
    println!("  # ...");
    println!("  # ...");
    println!("  \"set -x {} {}\"", ENV_KEY, config.config.path);
    println!("end");
    println!("_____________________________________");
}
