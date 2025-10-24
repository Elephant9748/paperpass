use crate::errors::err::{Error, message};
use colored::Colorize;
use std::path::Path;

pub static GPG_BIN: &str = "/usr/bin/gpg";
pub static AWK_BIN: &str = "/usr/bin/awk";
pub static ECHO_BIN: &str = "/usr/bin/awk";
pub static GIT_BIN: &str = "/usr/bin/git";

#[allow(clippy::box_collection)]
pub fn bin_in_box() -> Result<Box<Vec<&'static str>>, String> {
    let mut run_bin = Box::new(Vec::new());

    if Path::new(GPG_BIN).exists()
        && Path::new(AWK_BIN).exists()
        && Path::new(ECHO_BIN).exists()
        && Path::new(GIT_BIN).exists()
    {
        run_bin.push("gpg");
        run_bin.push("awk");
        run_bin.push("echo");
        run_bin.push("git");
    } else {
        return Err(format!("{}", message(Error::BinariesNotFound).bright_red()));
    }

    Ok(run_bin)
}
