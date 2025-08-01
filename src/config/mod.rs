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
struct Configs {
    config: Config,
    gpg: Gpg,
    store: Store,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gpg {
    key: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    path: String,
    git: bool,
}
#[derive(Serialize, Deserialize, Debug)]
struct Store {
    path: String,
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
        "\x1B[3m{}\x1B[23m",
        "   example path: ~/[whereyoustoredata] or just full path /[whereyoustoredata]"
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
    println!(
        "{:#?}\n{}{}",
        config,
        "::".bright_blue(),
        " init config succeed.".bright_green()
    );
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
    println!("{}", forcepath);
    // data store
    let store_path = set_store_path(opt1.to_string()).unwrap();

    let config = Configs {
        config: Config {
            path: format!("{}/paperpass.toml", forcepath),
            git: false,
        },
        gpg: Gpg {
            key: opt3.to_string(),
        },
        store: Store { path: store_path },
    };
    let toml = toml::to_string(&config).unwrap();
    let file =
        File::create(format!("{}paperpass.toml", forcepath)).expect(":: paperpass.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(toml.as_bytes());
    let _ = buf_writer.flush();
    println!(
        "{:#?}\n{}{}",
        config,
        "::".bright_blue(),
        " init config succeed.".bright_green()
    );
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
