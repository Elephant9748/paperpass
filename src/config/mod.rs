pub mod configfile;
pub mod pgpconfig;

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use colored::Colorize;

use crate::{
    catch_stdin,
    config::pgpconfig::{Config, Gpg},
    gpg::helper::{GpgHelper, listprivatekeys},
};

pub fn init_config() {
    let recipient = GpgHelper::new(listprivatekeys().unwrap());
    println!("\n{} {}\n", "::".bright_blue(), "Recipient".yellow());
    for (mut key, val) in recipient.get_all().unwrap().into_iter().enumerate() {
        *&mut key += 1;
        println!(
            "{}{}{} {}",
            "[".bright_purple(),
            key.to_string().bright_green(),
            "]".bright_purple(),
            val
        );
    }
    println!();
    print!("{}", "which key to use: ".bright_white());
    let input_key = catch_stdin();

    let config = Config {
        gpg: Gpg {
            key: format!(
                "{}",
                &recipient.get_all().unwrap()[input_key.parse::<usize>().unwrap() - 1]
            )
            .to_string(),
        },
    };
    let toml = toml::to_string(&config).unwrap();
    let file = File::create("paperpass.toml").expect(":: paperpass.toml not found");
    let mut buf_writer = BufWriter::new(file);
    let _ = buf_writer.write_all(toml.as_bytes());
    let _ = buf_writer.flush();
    println!(
        "\n{}{}",
        "::".bright_blue(),
        " write to paperpass.toml succeed.".bright_green()
    );
}
