pub fn prompt_help() {
    println!("USAGE:");
    println!("      - paperpass [GLOBAL OPTIONS]");
    println!("      - paperpass [GLOBAL ARG]");
    println!("      - paperpass [GLOBAL OPTIONS] [arg]");
    println!();
    println!("GLOBAL OPTIONS: ");
    println!("\n\tinit : Set Init Config");
    println!("\t  - init [arg]");
    println!("\t      arg:");
    println!("\t          -s  : where gpg store");
    println!("\t          -c  : where config saved");
    println!("\t          -pk : with pgp key by uid");
    println!("\n\tinsert : Set Init Config");
    println!("\t  - insert [path]");
    println!("\t      path: example [your/path]\n");
    println!("GLOBAL ARG: ");
    println!("\t-list :  List of Pgp Keys");
    println!("\t-h    :  Help");
    println!("\t-v    :  Version");
}
