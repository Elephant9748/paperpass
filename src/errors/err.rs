use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum PaperpassError {
    #[error(" option not found \"-h\" to see available options")]
    OptionsNotFound,
    #[error(":: catch_stdin failed.")]
    CatchStdin,
    #[error(":: File Not Found!")]
    FileNotFound,
    #[error(":: Error while read contents from bufreader!")]
    CantBufRead,
    #[error(":: ResultError on")]
    ResultConfig,
    #[error(":: Env not Found")]
    EnvNotFound,
    #[error("Binaries Doesnt Exists.")]
    BinariesNotFound,
    #[error("Copy to clipboard failed.")]
    CopyClipFailed,
}
