use crate::{
    config::{init_config, init_config_with_params},
    gpg::helper::{GpgHelper, listprivatekeys},
    options::helpstdout::prompt_help,
    utils::{
        clipboard::clipboard_copy, edit::edit_with_params, insert::insert_with_params,
        ls::list_dir_with_params, show::show_with_params, totp::totp_create,
    },
};

pub enum Opt {
    Init,
    InitParams(String, String, String, String),
    InsertParams(String),
    EditParams(String),
    ShowParams(String),
    Copy(String, i32),
    ListRecepients(String),
    TotpCreate(String, i32),
    ListDir(String),
    Help,
    Version,
}

pub fn args_options(opt: Opt) {
    match opt {
        Opt::Init => init_config(),
        Opt::InitParams(a, b, c, d) => {
            init_config_with_params(a.as_str(), b.as_str(), c.as_str(), d.as_str())
        }
        Opt::InsertParams(a) => insert_with_params(a.as_str()),
        Opt::EditParams(a) => edit_with_params(a.as_str()),
        Opt::ShowParams(a) => show_with_params(a.as_str()),
        Opt::Copy(a, b) => clipboard_copy(a.as_str(), b),
        Opt::TotpCreate(a, b) => totp_create(a.as_str(), b),
        Opt::ListDir(a) => list_dir_with_params(a.as_str()),
        Opt::ListRecepients(_a) => {
            let pk = GpgHelper::new(listprivatekeys().unwrap());
            println!("{:?}", pk.get_by_name("brandon"));
            println!("{:?}", pk.get_all());
        }
        Opt::Help => prompt_help(),
        Opt::Version => {
            let version = env!("CARGO_PKG_VERSION");
            let build_date = env!("DATE");
            let git_head_hash = env!("GIT_HASH");
            println!("paperpass {} ({} {})", version, git_head_hash, build_date);
        }
    }
}
