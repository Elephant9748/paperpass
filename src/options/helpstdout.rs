pub fn prompt_help() {
    println!("USAGE:");
    println!("      - paperpass [GLOBAL OPTIONS]");
    println!("      - paperpass [GLOBAL ARG]");
    println!("      - paperpass [GLOBAL OPTIONS] [arg]");
    println!();
    println!("GLOBAL OPTIONS: ");
    println!("\tinit : set init config");
    println!("\t  - init [arg]");
    println!("\t      arg:");
    println!("\t          -s  : where gpg store");
    println!("\t          -c  : where config saved");
    println!("\t          -pk : with pgp key by uid");
    println!("\tinsert :");
    println!("\t          - insert [path]");
    println!("\t            path: example [your/path]");
    println!("\tshow   :");
    println!("\t          - show [path]");
    println!("\t            path: example [your/path]");
    println!("GLOBAL ARG: ");
    println!("\t-list :  List of Pgp Keys");
    println!("\t-h    :  Help");
    println!("\t-v    :  Version");
}
