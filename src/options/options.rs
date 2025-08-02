use crate::{
    config::{init_config, init_config_with_params},
    gpg::helper::{GpgHelper, listprivatekeys},
    options::helpstdout::prompt_help,
    utils::{clipboard::clipboard_copy, insert::insert_with_params, show::show_with_params},
};

pub enum Opt {
    Init,
    InitParams(String, String, String),
    InsertParams(String),
    ShowParams(String),
    Copy(String),
    ListRecepients(String),
    Help,
    Version,
}

pub fn args_options(opt: Opt) {
    match opt {
        Opt::Init => init_config(),
        Opt::InitParams(a, b, c) => init_config_with_params(a.as_str(), b.as_str(), c.as_str()),
        Opt::InsertParams(a) => insert_with_params(a.as_str()),
        Opt::ShowParams(a) => show_with_params(a.as_str()),
        Opt::Copy(a) => clipboard_copy(a.as_str()),
        Opt::ListRecepients(_a) => {
            let pk = GpgHelper::new(listprivatekeys().unwrap());
            println!("{:?}", pk.get_by_name("brandon"));
            println!("{:?}", pk.get_all());
        }
        Opt::Help => prompt_help(),
        Opt::Version => {
            let version = env!("CARGO_PKG_VERSION");
            let build_date = env!("DATE");
            println!("paperpaer {} ({})", version, build_date);
        }
    }
}
