use crate::{
    config::pgpconfig::config_key_to_used,
    gpg::helper::{GpgHelper, listprivatekeys},
    options::helpstdout::prompt_help,
};

pub enum Opt {
    Init,
    ListRecepients(String),
    Help,
    Version,
}

pub fn args_options(opt: Opt) {
    match opt {
        Opt::Init => config_key_to_used(),
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
