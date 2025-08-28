use colored::Colorize;

pub fn prompt_help() {
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
        "\t\tCopy to clipboard"
    );
    println!(
        "{}{}",
        "     --time <TIME>".cyan(),
        "\t\tCopy to clipboard with autoclear after amount of sec TIME=NUMBER default is 30 sec"
    );
    println!("{}{}", "  -h".cyan(), "\t\t\t\tPrint help");
    println!("{}{}", "  -v".cyan(), "\t\t\t\tPrint version");
    println!("{}{}", "  -lk".cyan(), "\t\t\t\tPrint list of pgpkey");
    println!("{}{}", "  -config".cyan(), "\t\t\tShow config json");
    println!("{}", "\nCommand: ".green());
    println!(
        "{}{}",
        "  init".cyan(),
        "\t\t\t\tSet init config generate toml"
    );
    println!(
        "{}{}",
        "      -c ~/<YOUR CONFIG PATH>".cyan(),
        "\tWhere config saved, must have ~/ or the full path"
    );
    println!(
        "{}{}",
        "      -s ~/<YOUR DATA STORE>".cyan(),
        "\tWhere data store must have ~/ or the full path"
    );
    println!(
        "{}{}",
        "      -pk <YOUR PGP KEY>".cyan(),
        "\tSet which key to used (name of key)"
    );
    println!(
        "{}{}",
        "  insert  <YOUR/DATA/STORE>".cyan(),
        "\tInsert new secret."
    );
    println!(
        "{}{}",
        "  edit    <YOUR/DATA/STORE>".cyan(),
        "\tEdit secret."
    );
    println!(
        "{}{}",
        "  delete  <YOUR/DATA/STORE>".cyan(),
        "\tDelete a secret."
    );
    println!(
        "{}{}",
        "  show -s <YOUR/DATA/STORE>".cyan(),
        "\tShow secret"
    );
    println!(
        "{}{}",
        "  totp    <YOUR/DATA/STORE>".cyan(),
        "\tDisplay totp every 30"
    );
    println!(
        "{}{}",
        "  totp -c <YOUR/DATA/STORE>".cyan(),
        "\tCopy totp into clipboard"
    );
    println!(
        "{}{}",
        "  ls      <YOUR/DATA/STORE>".cyan(),
        "\tList of secrets"
    );
    println!(
        "\n{}paperpass {} ({} {})\n",
        "Version: ".green(),
        version,
        git_head_hash,
        build_date
    );
}
