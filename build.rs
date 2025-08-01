use chrono::{TimeZone, Utc};
use std::env;

fn main() {
    let build_date = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => Utc
            .timestamp_opt(val.parse::<i64>().unwrap(), 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
        Err(_) => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    println!("cargo:rustc-env=DATE={}", build_date);
}
