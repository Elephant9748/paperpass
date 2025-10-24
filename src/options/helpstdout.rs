use colored::Colorize;

pub fn prompt_help() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("DATE");
    let git_head_hash = env!("GIT_HASH");
    print!("{}", "\nUsage: ".green());
    println!("{}", "paperpass [COMMAND] [OPTIONS] [ARGS]".cyan());
    println!("{}", "       paperpass [GLOBAL OPTIONS] [ARGS]".cyan());
    println!("{}", "\nOptions global: ".green());
    println!(
        "{}{}",
        "  -c <YOUR/DATA/STORE>".cyan(),
        "\t\tCopy to clipboard".white()
    );
    println!(
        "{}{}",
        "     -time <TIME>".cyan(),
        "\t\tCopy to clipboard with autoclear after amount of sec TIME=NUMBER default is 30 sec"
            .white()
    );
    println!("{}{}", "  -h --help".cyan(), "\t\t\tPrint help".white());
    println!(
        "{}{}",
        "  -v --version".cyan(),
        "\t\t\tPrint version".white()
    );
    println!(
        "{}{}",
        "  -lk".cyan(),
        "\t\t\t\tPrint list of pgpkey".white()
    );
    println!(
        "{}{}",
        "  -config --config".cyan(),
        "\t\tShow config json".white()
    );
    println!("{}", "\nCommand: ".green());
    println!(
        "{}{}",
        "  init".cyan(),
        "\t\t\t\tSet init config generate toml".white()
    );
    println!(
        "{}{}",
        "      -c ~/<YOUR CONFIG PATH>".cyan(),
        "\tWhere config saved, must have ~/ or the full path".white()
    );
    println!(
        "{}{}",
        "      -s ~/<YOUR DATA STORE>".cyan(),
        "\tWhere data store must have ~/ or the full path".white()
    );
    println!(
        "{}{}",
        "      -pk <YOUR PGP KEY>".cyan(),
        "\tSet which key to used (name of key)".white()
    );
    println!(
        "{}{}",
        "  insert  <YOUR/DATA/STORE>".cyan(),
        "\tInsert new secret.".white()
    );
    println!(
        "{}{}",
        "  edit    <YOUR/DATA/STORE>".cyan(),
        "\tEdit secret.".white()
    );
    println!(
        "{}{}",
        "  delete  <YOUR/DATA/STORE>".cyan(),
        "\tDelete a secret.".white()
    );
    println!(
        "{}{}",
        "  show -s <YOUR/DATA/STORE>".cyan(),
        "\tShow secret".white()
    );
    println!(
        "{}{}",
        "  totp    <YOUR/DATA/STORE>".cyan(),
        "\tDisplay totp every 30".white()
    );
    println!(
        "{}{}",
        "  totp -c <YOUR/DATA/STORE>".cyan(),
        "\tCopy totp into clipboard".white()
    );
    println!("{}{}", "  ls".cyan(), "\t\t\t\tList of secrets".white());
    println!(
        "{}{}",
        "  ls      <YOUR/DATA/STORE>".cyan(),
        "\tList of secrets".white()
    );
    println!(
        "\n{}{} {} ({} {})\n",
        "Version: ".green(),
        name,
        version,
        git_head_hash,
        build_date
    );
}
