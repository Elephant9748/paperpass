# paperpass
> [!WARNING]
> not ready to be used 
## Usage
```
Usage: paperpass [COMMAND] [OPTIONS] [ARGS]
       paperpass [GLOBAL OPTIONS] [ARGS]

Options global:
  -c <YOUR/DATA/STORE>          Copy password to clipboard
  -time <TIME>                  Copy to clipboard with autoclear after amount of sec TIME=NUMBER default is 30 sec
  -h --help                     Print help
  -v --version                  Print version
  -lk                           Print list of pgpkey
  -config --config              Show config json

Command:
  init                                  Set init config generate toml
        -c      ~/<YOUR CONFIG PATH>    Where config saved, must have ~/ or the full path
        -s      ~/<YOUR DATA STORE>     Where data store must have ~/ or the full path
        -pk     <YOUR GPG KEY>          Set which key to used (name of key)
  user          <YOUR/DATA/STORE>       Show only username
        -c      <YOUR/DATA/STORE>       Copy username into clipboard
  insert        <YOUR/DATA/STORE>       Insert new secret.
  edit          <YOUR/DATA/STORE>       Edit secret.
  delete        <YOUR/DATA/STORE>       Delete a secret.
  show          <YOUR/DATA/STORE>       Show secret
  totp          <YOUR/DATA/STORE>       Display totp every 30
        -c      <YOUR/DATA/STORE>       Copy totp into clipboard
  ls                                    List of secrets
  ls            <YOUR/DATA/STORE>       List of secrets
  migrate       <YOUR GPG KEY NAME>     Migrate boxpaperpass to new key
        -d      <SOURCE PATH>           Contain source path
        -t      <DEST PATH>             Contain dest path
  genpass       <LENGTH>                Generate Passwords !not include Extended ASCII
```

### format doc inside encrypted file
```
[password or data to encrypt]
[username]
[otpauth url]
..
etc
```
