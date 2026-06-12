use crate::{
    errors::err::PaperpassError,
    options::opt::{Opt, args_options},
};
use colored::Colorize;

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Args {
    params: Vec<String>,
    run: RunArgs,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub enum RunArgs {
    #[default]
    Init,
    Insert,
    Show,
    Edit,
    Delete,
    User,
    Copy,
    Totp,
    ListVault,
    GenPass,
    Import,
    Migrate,
    ShowConfig,
    ListKeyAvailable,
    Version,
    Help,
    Empty,
}

#[allow(unused)]
impl Args {
    pub fn new(envarg: Vec<String>) -> Self {
        Self {
            params: envarg,
            run: RunArgs::default(),
        }
    }
    pub fn set_run(&mut self) {
        match &self.params {
            x if x.contains(&"init".to_string()) => self.run = RunArgs::Init,
            x if x.contains(&"insert".to_string()) => self.run = RunArgs::Insert,
            x if x.contains(&"edit".to_string()) => self.run = RunArgs::Edit,
            x if x.contains(&"user".to_string()) && x.len() == 2 => self.run = RunArgs::User,
            x if x.contains(&"show".to_string()) => self.run = RunArgs::Show,
            x if x.contains(&"delete".to_string()) => self.run = RunArgs::Delete,
            x if x.contains(&"totp".to_string()) && x.len() == 2 => self.run = RunArgs::Totp,
            x if x.contains(&"ls".to_string()) => self.run = RunArgs::ListVault,
            x if x.contains(&"genpass".to_string()) => self.run = RunArgs::GenPass,
            x if x.contains(&"migrate".to_string()) => self.run = RunArgs::Migrate,
            x if x.contains(&"import".to_string()) => self.run = RunArgs::Import,
            x if x.contains(&"-config".to_string()) => self.run = RunArgs::ShowConfig,
            x if x.contains(&"--config".to_string()) => self.run = RunArgs::ShowConfig,
            x if x.contains(&"-lk".to_string()) => self.run = RunArgs::ListKeyAvailable,
            x if x.contains(&"-c".to_string()) => self.run = RunArgs::Copy,
            x if x.contains(&"-c".to_string()) && x.contains(&"user".to_string())
                || x.contains(&"totp".to_string()) =>
            {
                self.run = RunArgs::Copy
            }
            x if x.contains(&"-v".to_string()) => self.run = RunArgs::Version,
            x if x.contains(&"--version".to_string()) => self.run = RunArgs::Version,
            x if x.contains(&"--help".to_string()) => self.run = RunArgs::Help,
            x if x.contains(&"-h".to_string()) => self.run = RunArgs::Help,
            _ => self.run = RunArgs::Empty,
        }
    }

    // main run
    pub fn run_args(&mut self) {
        match self.run {
            RunArgs::Version => {
                args_options(Opt::Version);
            }
            RunArgs::ListKeyAvailable => {
                args_options(Opt::ListRecepients(String::from("some text")))
            }
            RunArgs::ShowConfig => {
                args_options(Opt::ShowConfig);
            }
            RunArgs::Insert => {
                let index = self.index_params("insert");
                args_options(Opt::InsertParams(self.params[index + 1].to_owned()))
            }
            RunArgs::Edit => {
                let index = self.index_params("edit");
                args_options(Opt::EditParams(self.params[index + 1].to_owned()))
            }
            RunArgs::User => {
                let index = self.index_params("user");
                match self.params.len() {
                    2 => args_options(Opt::UserShow(self.params[index + 1].to_owned())),
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::Show => {
                let index = self.index_params("show");
                args_options(Opt::ShowParams(self.params[index + 1].to_owned()))
            }
            RunArgs::Delete => {
                let index = self.index_params("delete");
                args_options(Opt::DeleteParams(self.params[index + 1].to_owned()))
            }
            RunArgs::Copy => {
                let index = self.index_params("-c");
                let index_time = self.index_params("-time");
                let index_user = self.index_params("user");
                let index_totp = self.index_params("totp");
                match self.params.len() {
                    2 => {
                        if index < 1 {
                            // default clear clipboard timeout is 30sec
                            args_options(Opt::Copy(self.params[index + 1].to_owned(), 30))
                        } else {
                            // default clear clipboard timeout is 30sec
                            args_options(Opt::Copy(self.params[0].to_owned(), 30))
                        }
                    }
                    3 => {
                        match &self.params {
                            c if c.contains(&"user".to_string())
                                && c.contains(&"-c".to_string())
                                && index == 1 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::UserCopy(
                                    self.params[index_user + 2].to_owned(),
                                    30,
                                ));
                            }
                            c if c.contains(&"user".to_string())
                                && c.contains(&"-c".to_string())
                                && index == 2 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::UserCopy(
                                    self.params[index_user + 1].to_owned(),
                                    30,
                                ));
                            }
                            c if c.contains(&"totp".to_string())
                                && c.contains(&"-c".to_string())
                                && index == 1 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::TotpCreate(
                                    self.params[index_totp + 2].to_owned(),
                                    30,
                                ));
                            }
                            c if c.contains(&"totp".to_string())
                                && c.contains(&"-c".to_string())
                                && index == 2 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::TotpCreate(
                                    self.params[index_totp + 1].to_owned(),
                                    30,
                                ));
                            }
                            _ => {
                                self.run = RunArgs::Empty;
                                self.run_args();
                            }
                        }
                    }
                    5 => {
                        match &self.params {
                            c if c.contains(&"user".to_string())
                                && c.contains(&"-c".to_string())
                                && c.contains(&"-time".to_string())
                                && index == 1 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::UserCopy(
                                    self.params[index_user + 2].to_owned(),
                                    self.params[index_time + 1].parse::<i32>().unwrap_or_else(
                                        |_| panic!("> cant parse i32 on GenPass timeout"),
                                    ),
                                ));
                            }
                            c if c.contains(&"user".to_string())
                                && c.contains(&"-c".to_string())
                                && c.contains(&"-time".to_string())
                                && index == 2 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::UserCopy(
                                    self.params[index_user + 1].to_owned(),
                                    self.params[index_time + 1].parse::<i32>().unwrap_or_else(
                                        |_| panic!("> cant parse i32 on GenPass timeout"),
                                    ),
                                ));
                            }
                            c if c.contains(&"totp".to_string())
                                && c.contains(&"-c".to_string())
                                && c.contains(&"-time".to_string())
                                && index == 1 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::TotpCreate(
                                    self.params[index_totp + 2].to_owned(),
                                    self.params[index_time + 1].parse::<i32>().unwrap_or_else(
                                        |_| panic!("> cant parse i32 on GenPass timeout"),
                                    ),
                                ));
                            }
                            c if c.contains(&"totp".to_string())
                                && c.contains(&"-c".to_string())
                                && c.contains(&"-time".to_string())
                                && index == 2 =>
                            {
                                // default clear clipboard timeout is 30sec
                                args_options(Opt::TotpCreate(
                                    self.params[index_totp + 1].to_owned(),
                                    self.params[index_time + 1].parse::<i32>().unwrap_or_else(
                                        |_| panic!("> cant parse i32 on GenPass timeout"),
                                    ),
                                ));
                            }
                            _ => {
                                self.run = RunArgs::Empty;
                                self.run_args();
                            }
                        }
                    }
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::Totp => {
                let index = self.index_params("totp");
                let index_b = self.index_params("-c");
                match self.params.len() {
                    2 => args_options(Opt::TotpCreate(self.params[index + 1].to_owned(), 0)),
                    3 => {
                        if self.params.contains(&"totp".to_string())
                            && self.params.contains(&"-c".to_string())
                            && index_b == 1
                        {
                            args_options(Opt::TotpCreate(self.params[index_b + 1].to_owned(), 30));
                        } else {
                            args_options(Opt::TotpCreate(self.params[index + 1].to_owned(), 30));
                        }
                    }
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::ListVault => {
                let index = self.index_params("ls");
                if self.params.len() > 1 {
                    args_options(Opt::ListDir(self.params[index + 1].to_owned()))
                } else {
                    args_options(Opt::ListDirRoot)
                }
            }
            RunArgs::GenPass => {
                let index = self.index_params("genpass");
                let index_b = self.index_params("-time");
                match self.params.len() {
                    2 => args_options(Opt::GenPass(
                        self.params[index + 1]
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("> cant parse i32 on GenPass length")),
                        30,
                    )),
                    4 => args_options(Opt::GenPass(
                        self.params[index + 1]
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("> cant parse i32 on GenPass length")),
                        self.params[index_b + 1]
                            .parse::<i32>()
                            .unwrap_or_else(|_| panic!("> cant parse i32 on GenPass timeout")),
                    )),
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::Import => {
                let index = self.index_params("import");
                args_options(Opt::Import(self.params[index + 1].to_owned()))
            }
            RunArgs::Init => {
                let index_a = self.index_params("-s");
                let index_b = self.index_params("-c");
                let index_c = self.index_params("-pk");
                match self.params.len() {
                    1 => args_options(Opt::Init),
                    2 => {
                        if self.params.contains(&"-git".to_string())
                            && self.params.contains(&"init".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                "".to_owned(),
                                "".to_owned(),
                                "y".to_owned(),
                            ));
                        } else {
                            self.run = RunArgs::Empty;
                            self.run_args();
                        }
                    }
                    3 => {
                        if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-s".to_string())
                        {
                            args_options(Opt::InitParams(
                                self.params[index_a + 1].to_owned(),
                                "".to_owned(),
                                "".to_owned(),
                                "".to_owned(),
                            ));
                        } else if self.params.contains(&"init".to_string())
                            && self.params.contains(&"c".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                self.params[index_b + 1].to_owned(),
                                "".to_owned(),
                                "".to_owned(),
                            ));
                        } else if self.params.contains(&"init".to_string())
                            && self.params.contains(&"pk".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                "".to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "".to_owned(),
                            ));
                        } else {
                            self.run = RunArgs::Empty;
                            self.run_args();
                        }
                    }
                    4 => {
                        if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-pk".to_string())
                            && self.params.contains(&"-git".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                "".to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "".to_owned(),
                            ));
                        } else {
                            self.run = RunArgs::Empty;
                            self.run_args();
                        }
                    }
                    5 => {
                        if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-s".to_string())
                            && self.params.contains(&"-c".to_string())
                        {
                            args_options(Opt::InitParams(
                                self.params[index_a + 1].to_owned(),
                                self.params[index_b + 1].to_owned(),
                                "".to_owned(),
                                "".to_owned(),
                            ));
                        } else if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-s".to_string())
                            && self.params.contains(&"-pk".to_string())
                        {
                            args_options(Opt::InitParams(
                                self.params[index_a + 1].to_owned(),
                                "".to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "".to_owned(),
                            ));
                        } else if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-c".to_string())
                            && self.params.contains(&"-pk".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                self.params[index_b + 1].to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "".to_owned(),
                            ));
                        } else {
                            self.run = RunArgs::Empty;
                            self.run_args();
                        }
                    }
                    6 => {
                        if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-pk".to_string())
                            && self.params.contains(&"-s".to_string())
                            && self.params.contains(&"-git".to_string())
                        {
                            args_options(Opt::InitParams(
                                "".to_owned(),
                                self.params[index_b + 1].to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "y".to_owned(),
                            ));
                        } else if self.params.contains(&"init".to_string())
                            && self.params.contains(&"-pk".to_string())
                            && self.params.contains(&"-c".to_string())
                            && self.params.contains(&"-git".to_string())
                        {
                            args_options(Opt::InitParams(
                                self.params[index_a + 1].to_owned(),
                                "".to_owned(),
                                self.params[index_c + 1].to_owned(),
                                "y".to_owned(),
                            ));
                        } else {
                            self.run = RunArgs::Empty;
                            self.run_args();
                        }
                    }
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::Migrate => {
                let index_a = self.index_params("migrate");
                let index_b = self.index_params("-d");
                let index_c = self.index_params("-t");
                match self.params.len() {
                    2 => {
                        args_options(Opt::Migrate(self.params[index_a + 1].to_owned()));
                    }
                    6 => {
                        args_options(Opt::MigrateExternal(
                            self.params[index_a + 1].to_owned(),
                            self.params[index_b + 1].to_owned(),
                            self.params[index_c + 1].to_owned(),
                        ));
                    }
                    _ => {
                        self.run = RunArgs::Empty;
                        self.run_args();
                    }
                }
            }
            RunArgs::Help => args_options(Opt::Help),
            RunArgs::Empty => println!(
                "{}{}",
                "::".bright_blue(),
                PaperpassError::OptionsNotFound.to_string().bright_yellow()
            ),
        }
    }

    fn index_params(&self, args: &str) -> usize {
        if self.params.contains(&args.to_string()) {
            self.params.iter().position(|mark| mark == args).unwrap()
        } else {
            0
        }
    }
}
