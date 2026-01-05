use colored::Colorize;

use crate::{
    config::{init_config, init_config_with_params, showconfig::show_config},
    gpg::helper::{GpgHelper, listprivatekeys},
    options::helpstdout::prompt_help,
    utils::{
        clipboard::{clipboard_copy, username_copy, username_show},
        delete::delete_with_params,
        edit::edit_with_params,
        insert::insert_with_params,
        ls::{list_dir_root, list_dir_with_params},
        migrate::{send_to_another_box, send_to_another_box_external},
        show::show_with_params,
        totp::totp_create,
    },
};

pub enum Opt {
    Init,
    InitParams(String, String, String, String),
    InsertParams(String),
    EditParams(String),
    ShowParams(String),
    DeleteParams(String),
    Copy(String, i32),
    UserCopy(String, i32),
    UserShow(String),
    ShowConfig,
    ListRecepients(String),
    TotpCreate(String, i32),
    ListDir(String),
    ListDirRoot,
    Migrate(String),
    MigrateExternal(String, String, String),
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
        Opt::DeleteParams(a) => delete_with_params(a.as_str()),
        Opt::Copy(a, b) => clipboard_copy(a.as_str(), b),
        Opt::UserCopy(a, b) => username_copy(a.as_str(), b),
        Opt::UserShow(a) => username_show(a.as_str()),
        Opt::TotpCreate(a, b) => totp_create(a.as_str(), b),
        Opt::ListDir(a) => list_dir_with_params(a.as_str()),
        Opt::ListDirRoot => list_dir_root(),
        Opt::ShowConfig => show_config(),
        Opt::ListRecepients(_a) => {
            let pk = GpgHelper::new(listprivatekeys().unwrap());
            println!(
                "{}",
                "Depend on your gpg key available in ~/.gnupg".bright_cyan()
            );
            println!("{}", "Available pgp key: ".bright_green());
            let mut i = 1;
            for k in pk.get_all().unwrap().iter() {
                println!(
                    "\t\t{}{}{} {}",
                    "[".bright_yellow(),
                    i.to_string().bright_blue(),
                    "]".bright_yellow(),
                    k.bright_cyan()
                );
                i += 1;
            }
        }
        Opt::Migrate(params) => send_to_another_box(params),
        Opt::MigrateExternal(p1, p2, p3) => send_to_another_box_external(p1, p2, p3),
        Opt::Help => prompt_help(),
        Opt::Version => {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            let build_date = env!("DATE");
            let git_head_hash = env!("GIT_HASH");
            println!("{} {} ({} {})", name, version, git_head_hash, build_date);
        }
    }
}
