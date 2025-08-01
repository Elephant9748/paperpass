#[allow(dead_code)]
pub enum Error {
    OptionsNotFound,
    CatchStdin,
    FileNotFound,
    CantBufRead,
    ResultError,
    EnvNotFound,
}

pub fn message(err: Error) -> String {
    match err {
        Error::OptionsNotFound => " option not found \"-h\" to see available options".to_string(),
        Error::CatchStdin => ":: catch_stdin failed.".to_string(),
        Error::FileNotFound => ":: File Not Found!".to_string(),
        Error::CantBufRead => ":: Error while read contents from bufreader!".to_string(),
        Error::ResultError => ":: ResultError on".to_string(),
        Error::EnvNotFound => ":: Env not Found".to_string(),
    }
}
