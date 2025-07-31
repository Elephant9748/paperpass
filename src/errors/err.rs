pub enum Error {
    OptionsNotFound,
    CatchStdin,
}

pub fn message(err: Error) -> String {
    match err {
        Error::OptionsNotFound => " option not found \"-h\" to see available options".to_string(),
        Error::CatchStdin => ":: catch_stdin failed.".to_string(),
    }
}
