use crate::{
    config::{configfile::home_path, init_config_with_params},
    utils::{
        insert::insert_for_migration,
        ls::{Dirs, Ls},
        show::show_with_params_noprint,
    },
};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType, DisableLineWrap, size},
};
use std::{
    collections::BTreeMap,
    io::{Write, stdout},
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn send_to_another_box(params: String) {
    let lss = Arc::new(Mutex::new(Migrate::new(Ls::new("".to_string()))));
    let mut th_handles = Vec::new();

    let th1 = Arc::clone(&lss);
    let th_handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut lss_th = th1.lock().expect(":: lss.lock() failed");
        lss_th.set_fpath();
        lss_th.set_files();
    });
    th_handles.push(th_handle1);

    let th2 = Arc::clone(&lss);
    let th_handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut lss_th = th2.lock().expect(":: lss.lock() failed");
        lss_th.open_encrypted_files();
    });
    th_handles.push(th_handle2);

    for th in th_handles {
        th.join().expect(":: join thread failed");
    }

    let th3 = Arc::clone(&lss);
    let par = params.to_owned();
    let th_handle3 = thread::spawn(move || {
        let mut lss_th = th3.lock().expect(":: lss.lock() failed");
        lss_th.write_new_encrypted_files(par.as_str());
    });

    th_handle3.join().expect(":: join th_handle3 failed");

    // write new store path to config
    lss.lock()
        .expect(":: lls.lock() failed")
        .write_new_store_path(&params);

    // progress_crossterm();
}

pub fn send_to_another_box_external(params: String, dpath: String, tpath: String) {
    let lss = Arc::new(Mutex::new(Migrate::new(Ls::new("".to_string()))));
    let mut th_handles = Vec::new();

    let th1 = Arc::clone(&lss);
    let th_handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut lss_th = th1.lock().expect(":: lss.lock() failed");
        let valid_dpath = home_path(dpath.to_owned()).unwrap();
        let valid_tpath = home_path(tpath.to_owned()).unwrap();
        lss_th.set_files_external(&valid_dpath);
        lss_th.set_dest_target_path(valid_dpath, valid_tpath);
    });
    th_handles.push(th_handle1);

    let th2 = Arc::clone(&lss);
    let th_handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut lss_th = th2.lock().expect(":: lss.lock() failed");
        lss_th.open_encrypted_files();
    });
    th_handles.push(th_handle2);

    for th in th_handles {
        th.join().expect(":: join thread failed");
    }

    let th3 = Arc::clone(&lss);
    let par = params.to_owned();
    let th_handle3 = thread::spawn(move || {
        let mut lss_th = th3.lock().expect(":: lss.lock() failed");
        lss_th.write_new_encrypted_files(par.as_str());
    });

    th_handle3.join().expect(":: join th_handle3 failed");
}

#[derive(Debug, Clone)]
struct Migrate {
    ls: Ls,
    files: Vec<String>,
    open: BTreeMap<String, String>,
    fpath: i16,
    dpath: String,
    tpath: String,
}

impl Migrate {
    fn new(ls: Ls) -> Self {
        Self {
            ls,
            files: Vec::new(),
            open: BTreeMap::new(),
            fpath: 0,
            dpath: String::new(),
            tpath: String::new(),
        }
    }
    //all file in the store path into vector
    fn set_files(&mut self) {
        self.ls.get_store_path();
        let path = self.ls.store_path.to_owned();
        let mut dirs = Dirs::new(path.as_str());
        self.ls
            .get_to_dirs(&mut dirs, path.into())
            .expect(":: Failed get Dirs (send_to_another_box)");
        let files = dirs.flattern_dirs("");
        self.files = files;
    }
    fn set_files_external(&mut self, path: &str) {
        let mut dirs = Dirs::new(path);
        self.ls
            .get_to_dirs(&mut dirs, path.into())
            .expect(":: Failed get Dirs (send_to_another_box)");
        let files = dirs.flattern_dirs("");
        self.files = files.iter().map(|x| path.to_owned() + "/" + x).collect();
    }
    fn set_fpath(&mut self) {
        self.fpath = 1;
    }
    fn set_dest_target_path(&mut self, dpath: String, tpath: String) {
        self.dpath = dpath;
        self.tpath = tpath;
    }
    fn open_encrypted_files(&mut self) {
        let mut stdout = stdout();
        let (w, h) = size().unwrap();
        stdout.execute(DisableLineWrap).unwrap();
        stdout.execute(Hide).unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        #[allow(unused_assignments)]
        let mut t = 0;
        let total = &self.files.len();
        for (i, d) in self.files.iter().enumerate() {
            if i != 0 {
                t = (h - 4) as isize - (i) as isize;
            } else {
                t = (h - 4) as isize - 1;
            }

            if self.fpath == 1 {
                let d_replace = d.replace(".asc", "");
                let open = show_with_params_noprint(&d_replace, self.fpath);
                self.open.insert(d_replace.to_owned(), open);
            } else {
                let open = show_with_params_noprint(d, self.fpath);
                self.open.insert(d.to_owned(), open);
            }

            let prog = if i == (total - 1) {
                100_f32
            } else {
                ((i as f32 / *total as f32) * (100) as f32).floor()
            };

            let _ = execute!(
                stdout,
                MoveTo(0, h - 3),
                Clear(ClearType::FromCursorDown),
                SetForegroundColor(Color::Green),
                Print(":: located encrypted files\n"),
                SetForegroundColor(Color::Yellow),
                Print(format!(
                    "-> [{}{}{}] {}% {} file\n",
                    "-".repeat(i / (total / (total / 2))),
                    ">",
                    "".repeat((total / (total / (total / 2))) - (i / (total / (total / 2)))),
                    prog,
                    i + 1
                )),
            );
            // line
            let _ = execute!(
                stdout,
                MoveTo(0, h - 5),
                SetForegroundColor(Color::Yellow),
                Print("─".repeat(w as usize))
            );
            // pin the recent log to bottom
            let _ = execute!(
                stdout,
                MoveTo(0, h - 4),
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::Yellow),
                Print("->"),
                SetForegroundColor(Color::Blue),
                Print(format!(" {}\n", d)),
            );
            let _ = execute!(
                stdout,
                // (69-6)-0+1 =62 1
                // (69-6)-1+4 =59 5
                // (69-6)-2+7 =54 9
                // (69-6)-3+10 =49 13
                MoveTo(0, if t < 0 { 0 } else { t as u16 }),
                SetForegroundColor(Color::Yellow),
                Print("->"),
                SetForegroundColor(Color::Reset),
                Print(format!(" {}\n", d)),
            );
            stdout.flush().unwrap();
        }
        let _ = execute!(stdout, MoveTo(0, h + 5));
        stdout.flush().unwrap();
    }
    fn write_new_encrypted_files(&mut self, params: &str) {
        #[allow(unused_assignments)]
        let mut migrate_path = String::from("");
        if self.fpath == 0 {
            migrate_path = self.tpath.to_owned();
            if !Path::new(&migrate_path).exists() {
                std::fs::create_dir_all(&migrate_path)
                    .expect(":: failed to create directory for migrate_path");
            }
        } else {
            self.ls.get_store_path();
            let path = self.ls.store_path.to_owned();
            migrate_path = path + "_" + params;
            if !Path::new(&migrate_path).exists() {
                std::fs::create_dir_all(&migrate_path)
                    .expect(":: failed to create directory for migrate_path");
            }
        }

        let mut stdout = stdout();
        let (w, h) = size().unwrap();
        stdout.execute(DisableLineWrap).unwrap();
        stdout.execute(Hide).unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        let mut c = 0;
        #[allow(unused_assignments)]
        let mut t = 0;
        let total = self.open.len();
        for (i, (k, v)) in self.open.iter().enumerate() {
            if i != 0 {
                t = (h - 6) as isize - (i + c) as isize;
            } else {
                t = (h - 6) as isize - 1;
            }

            #[allow(unused_assignments)]
            let mut k1 = String::from("");
            if self.fpath == 0 {
                let x = k.replace(self.dpath.as_str(), "").replace(".asc", "");
                k1 = x;
            } else {
                k1 = k.to_string();
            }

            let prog = if i == (total - 1) {
                100_f32
            } else {
                ((i as f32 / total as f32) * (100) as f32).floor()
            };

            insert_for_migration(k1.as_str(), v, &migrate_path, params);
            let _ = execute!(
                stdout,
                MoveTo(0, h - 5),
                Clear(ClearType::FromCursorDown),
                SetForegroundColor(Color::Green),
                Print(":: write new encrypted files\n"),
                SetForegroundColor(Color::Yellow),
                Print(format!(
                    "-> [{}{}{}] {}% {} file\n",
                    "-".repeat(i / (total / (total / 2))),
                    ">",
                    "".repeat((total / (total / (total / 2))) - (i / (total / (total / 2)))),
                    prog,
                    i + 1
                )),
            );
            // line
            let _ = execute!(
                stdout,
                MoveTo(0, h - 8),
                SetForegroundColor(Color::Yellow),
                Print("─".repeat(w as usize))
            );
            // pin the recent log to bottom
            let _ = execute!(
                stdout,
                MoveTo(0, h - 7),
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::Yellow),
                Print("->"),
                SetForegroundColor(Color::Reset),
                Print(format!(" {}\n", k)),
            );
            let _ = execute!(stdout, MoveTo(0, h - 6), Clear(ClearType::CurrentLine));
            let _ = execute!(
                stdout,
                // (69-6)-0+1 =62 1
                // (69-6)-1+4 =59 5
                // (69-6)-2+7 =54 9
                // (69-6)-3+10 =49 13
                MoveTo(0, if t < 0 { 0 } else { t as u16 }),
                SetForegroundColor(Color::Yellow),
                Print("->"),
                SetForegroundColor(Color::Reset),
                Print(format!(" {}\n", k)),
            );
            match i {
                0 => c += 5,
                _ => c += 3,
            }
            stdout.flush().unwrap();
        }
        let _ = execute!(stdout, MoveTo(0, h + 5));
        stdout.flush().unwrap();
    }
    #[allow(dead_code)]
    fn write_new_store_path(&mut self, params: &str) {
        self.ls.get_store_path();
        let path = self.ls.store_path.to_owned();
        let new_path = path + "_" + params;
        init_config_with_params(new_path.as_str(), "", params, "");
    }
}

// check log and progress
#[allow(dead_code)]
fn progress_crossterm() {
    let mut stdout = stdout();
    let (w, h) = size().unwrap();
    stdout.execute(DisableLineWrap).unwrap();
    stdout.execute(Hide).unwrap();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let mut c = 0;
    #[allow(unused_assignments)]
    let mut t = 0;
    let mut dash = 100;
    let total = 22;

    // how many dash
    let mut prog_str = String::from("");
    for _ in 0..=dash / total {
        prog_str.push('-');
    }
    for i in 0..=total {
        if i != 0 {
            t = (h - 6) as isize - (i + c) as isize;
        } else {
            t = (h - 6) as isize - 1;
        }
        let _ = execute!(
            stdout,
            MoveTo(0, h - 3),
            Clear(ClearType::FromCursorDown),
            SetForegroundColor(Color::DarkBlue),
            Print(":: Write new encrypted files\n"),
            SetForegroundColor(Color::Cyan),
            Print(format!(
                ":: [{}{}{}] {}% {} file\n",
                // 0/(22/(22/2)) -> 0
                // 1/(22/(22/2)) -> 0.5
                // 2/(22/(22/2)) -> 1
                "-".repeat(i / (total / (total / 2))),
                ">",
                // (22/(22/(22/2))) - (0 / (22/(22/2)) -> 11
                // (100/(100/(100/2))) - (0 / (100/(100/2)) -> 50
                // (100/(100/(100/2))) - (1 / (100/(100/2)) -> 49.5
                " ".repeat((total / (total / (total / 2))) - (i / (total / (total / 2)))),
                // " ".repeat((w / 2) as usize),
                ((i as f32 / total as f32) * (100) as f32).floor(),
                i + 1
            )),
        );
        // pin log in bottom
        let _ = execute!(
            stdout,
            // (69-6)-0+1 =62 1
            // (69-6)-1+4 =59 5
            // (69-6)-2+7 =54 9
            // (69-6)-3+10 =49 13
            MoveTo(0, h - 7),
            SetForegroundColor(Color::Yellow),
            Print("[INFO]"),
            SetForegroundColor(Color::Reset),
            Print(format!(" progress {} h:{}", i, h)),
            SetForegroundColor(Color::Green),
            Print(format!(" Move:{} dash:{} w:{}\n", t, dash as isize, w)),
        );
        let _ = execute!(
            stdout,
            // (69-6)-0+1 =62 1
            // (69-6)-1+4 =59 5
            // (69-6)-2+7 =54 9
            // (69-6)-3+10 =49 13
            // MoveTo(0, if t < 0 { 0 } else { t as u16 }),
            MoveTo(0, if t < 0 { 0 } else { t as u16 }),
            SetForegroundColor(Color::Yellow),
            Print("[INFO]"),
            SetForegroundColor(Color::Reset),
            Print(format!(" progress {} h:{}", i, h)),
            SetForegroundColor(Color::Green),
            Print(format!(
                " Move:{} dash:{} w:{}\n",
                t,
                i / (dash / (dash / 2)),
                w
            )),
        );
        match i {
            0 => c += 1,
            _ => {
                c += 0;
                dash -= 100 / total;
            }
        }
        thread::sleep(Duration::from_millis(100));
        stdout.flush().unwrap();
    }
    let _ = execute!(stdout, MoveTo(0, h + 5));
    stdout.flush().unwrap();
}
