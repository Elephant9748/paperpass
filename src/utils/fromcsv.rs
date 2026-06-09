use crate::{
    config::configfile::set_config_path,
    gpg::lock::PaperCrypt,
    utils::{manage_env::ENV_CONFIG, read_config_file},
};
use crossterm::{
    cursor::{Hide, MoveToNextLine},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, size},
};
use serde::Deserialize;
use std::{
    env,
    error::Error,
    io::{self, Write, stdout},
    path::{self, PathBuf},
    process, thread,
    time::Duration,
};

// "Group","Title","Username","Password","URL","Notes","TOTP","Icon","Last Modified","Created"
#[derive(Debug, Deserialize)]
#[allow(unused)]
#[allow(non_snake_case)]
struct KeepassCsv {
    Group: String,
    Title: String,
    Username: String,
    Password: String,
    URL: String,
    Notes: String,
    TOTP: String,
}
fn keepass_import(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let defaultconfig =
        env::var(ENV_CONFIG).unwrap_or_else(|_| panic!(":: cant call ENV for keepass_import fn"));
    let dconfig = read_config_file(&defaultconfig).unwrap();

    let path_check_home = set_config_path(path.to_owned().into_os_string().into_string().unwrap());

    let mut rd = csv::Reader::from_path(path_check_home.to_owned().unwrap())?;
    let mut rd1 = csv::Reader::from_path(path_check_home.unwrap())?;

    let mut stdout = stdout();
    let (w, _h) = size().unwrap();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    let pac = ["C", "c"];
    let total = rd1.deserialize::<KeepassCsv>().count();
    let track = w as f32 * 0.25;
    for (ind, kpass) in rd.deserialize().enumerate() {
        let progress = ind as f32 / total as f32;
        let curpos = (progress * track) as usize;
        let pac_idx = ind % 2;
        let pacrun = pac[pac_idx];

        let kepass_record: KeepassCsv = kpass?;
        let _ = kepass_write_to_file(
            PathBuf::from(kepass_record.Group.to_owned()),
            PathBuf::from(&dconfig.store.path),
            kepass_record.Title.as_str(),
            kepass_record.Username.as_str(),
            kepass_record.Password.as_str(),
            kepass_record.TOTP.as_str(),
            dconfig.gpg.key.as_str(),
        );
        // println!("{:?}", kepass_record);
        let mut bar = String::new();
        for j in 0..track as usize {
            if j < curpos {
                bar.push('-');
            } else if j == curpos {
                bar.push_str(pacrun);
            } else {
                bar.push('*');
            }
        }
        execute!(
            stdout,
            MoveToNextLine(1),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Blue),
            Print(":: Keepass csv to papaperpass: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("[{}]{:3}%", bar, (progress * 100.0) as u32)),
            ResetColor
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

pub fn keepass_import_run(path: PathBuf) {
    if let Err(er) = keepass_import(path) {
        println!("error running keepass_import: {}", er);
        process::exit(1);
    } else {
        let mut stdout = stdout();
        execute!(
            stdout,
            MoveToNextLine(1),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Blue),
            Print(":: "),
            SetForegroundColor(Color::Yellow),
            Print("Keepass csv to papaperpass: "),
            SetForegroundColor(Color::Green),
            Print("Success"),
            ResetColor
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

fn kepass_write_to_file(
    group: PathBuf,
    destpath: PathBuf,
    title: &str,
    user: &str,
    pass: &str,
    totp: &str,
    keyname: &str,
) -> io::Result<()> {
    if let Ok(destpath) = destpath.to_owned().into_os_string().into_string()
        && let Ok(group) = group.to_owned().into_os_string().into_string()
    {
        let destp = destpath + "/" + &group;
        if !path::Path::new(&destp).exists() {
            std::fs::create_dir_all(&destp).expect(":: force_create_path() for config failed");
        }

        let file_path = destp + "/" + title.replace(" ", "_").as_str() + ".asc";
        let s_out = pass.to_owned() + "\n" + user + "\n" + totp;
        let mut papercrypt = PaperCrypt::newpure(&file_path, &s_out, keyname);
        papercrypt.encrypt_with_params_pure()?;
    }
    Ok(())
}
