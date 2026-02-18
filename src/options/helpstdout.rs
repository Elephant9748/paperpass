use crate::options::banner::prompt_banner;
use colored::Colorize;

pub fn prompt_help() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("DATE");
    let git_head_hash = env!("GIT_HASH");
    prompt_banner();
    print!("{}", "\nUsage: ".green());
    println!("{}", "paperpass [COMMAND] [OPTIONS] [ARGS]".cyan());
    println!("{}", "       paperpass [GLOBAL OPTIONS] [ARGS]".cyan());
    println!("{}", "\nOptions global: ".green());
    println!(
        "{}{}",
        "  -c <YOUR/DATA/STORE>".cyan(),
        "\t\tCopy password to clipboard".white()
    );
    println!(
        "{}{}",
        "  -time <TIME>".cyan(),
        "\t\t\tCopy to clipboard with autoclear after amount of sec TIME=NUMBER default is 30 sec"
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
        "\t\t\t\t\tSet init config generate toml".white()
    );
    println!(
        "{}{}",
        "\t-c\t~/<YOUR CONFIG PATH>".cyan(),
        "\tWhere config saved, must have ~/ or the full path".white()
    );
    println!(
        "{}{}",
        "\t-s\t~/<YOUR DATA STORE>".cyan(),
        "\tWhere data store must have ~/ or the full path".white()
    );
    println!(
        "{}{}",
        "\t-pk\t<YOUR GPG KEY>".cyan(),
        "\t\tSet which key to used (name of key)".white()
    );
    println!(
        "{}{}",
        "  user\t\t<YOUR/DATA/STORE>".cyan(),
        "\tShow only username".white()
    );
    println!(
        "{}{}",
        "\t-c\t<YOUR/DATA/STORE>".cyan(),
        "\tCopy username into clipboard".white()
    );
    println!(
        "{}{}",
        "  insert\t<YOUR/DATA/STORE>".cyan(),
        "\tInsert new secret.".white()
    );
    println!(
        "{}{}",
        "  edit\t\t<YOUR/DATA/STORE>".cyan(),
        "\tEdit secret.".white()
    );
    println!(
        "{}{}",
        "  delete\t<YOUR/DATA/STORE>".cyan(),
        "\tDelete a secret.".white()
    );
    println!(
        "{}{}",
        "  show\t\t<YOUR/DATA/STORE>".cyan(),
        "\tShow secret".white()
    );
    println!(
        "{}{}",
        "  totp\t\t<YOUR/DATA/STORE>".cyan(),
        "\tDisplay totp every 30".white()
    );
    println!(
        "{}{}",
        "\t-c\t<YOUR/DATA/STORE>".cyan(),
        "\tCopy totp into clipboard".white()
    );
    println!("{}{}", "  ls".cyan(), "\t\t\t\t\tList of secrets".white());
    println!(
        "{}{}",
        "  ls\t\t<YOUR/DATA/STORE>".cyan(),
        "\tList of secrets".white()
    );
    println!(
        "{}{}",
        "  migrate\t<YOUR GPG KEY NAME>".cyan(),
        "\tMigrate boxpaperpass to new key".white()
    );
    println!(
        "{}{}",
        "\t-d \t<SOURCE PATH>".cyan(),
        "\t\tContain source path".white()
    );
    println!(
        "{}{}",
        "\t-t \t<DEST PATH>".cyan(),
        "\t\tContain dest path".white()
    );
    println!(
        "{}{}",
        "  genpass\t<Length>".cyan(),
        "\t\tGenerate Passwords".white()
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
