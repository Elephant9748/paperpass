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
    println!("{}{}", "  -list".cyan(), "\t\t\t\tPrint list of pgpkey");
    println!("{}", "\nCommand: ".green());
    println!(
        "{}{}",
        "  init".cyan(),
        "\t\t\t\tSet init config generate toml"
    );
    println!(
        "{}{}",
        "      -c <YOUR/CONFIG/PATH>".cyan(),
        "\tWhere config saved"
    );
    println!(
        "{}{}",
        "      -s <YOUR/DATA/STORE>".cyan(),
        "\tWhere data store"
    );
    println!(
        "{}{}",
        "      -pk                 ".cyan(),
        "\tEncrypt data with apgp key by uid (name of key)"
    );
    println!(
        "{}{}",
        "  show -s <YOUR/DATA/STORE>".cyan(),
        "\tShow secret"
    );
    println!(
        "{}{}",
        "  totp <YOUR/DATA/STORE>".cyan(),
        "\tDisplay totp every 30"
    );
    println!(
        "{}{}",
        "  totp -c <YOUR/DATA/STORE>".cyan(),
        "\tCopy totp into clipboard"
    );
    println!(
        "\n{}paperpass {} ({} {})\n",
        "Version: ".green(),
        version,
        git_head_hash,
        build_date
    );
}
