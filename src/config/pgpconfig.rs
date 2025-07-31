use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gpg: Gpg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpg {
    pub key: String,
}
