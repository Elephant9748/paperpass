# paperpass
> [!WARNING]
> not ready to be used 
## Usage
```
Usage: paperpass [COMMAND] [OPTIONS] [ARGS]
       paperpass [GLOBAL OPTIONS] [ARGS]

Options global:
  -c <YOUR/DATA/STORE>          Copy to clipboard
     --time <TIME>              Copy to clipboard with autoclear after amount of sec TIME=NUMBER default is 30 sec
  -h                            Print help
  -v                            Print version
  -lk                           Print list of pgpkey
  -config                       Show config json

Command:
  init                          Set init config generate toml
      -c ~/<YOUR CONFIG PATH>   Where config saved, must have ~/ or the full path
      -s ~/<YOUR DATA STORE>    Where data store must have ~/ or the full path
      -pk <YOUR PGP KEY>        Set which key to used (name of key)
  insert  <YOUR/DATA/STORE>     Insert new secret.
  edit    <YOUR/DATA/STORE>     Edit secret.
  show -s <YOUR/DATA/STORE>     Show secret
  totp    <YOUR/DATA/STORE>     Display totp every 30
  totp -c <YOUR/DATA/STORE>     Copy totp into clipboard
  ls      <YOUR/DATA/STORE>     List of secrets
```

### format password with otpauth
```
[password or data to encrypt]
---
[otpauth url]
```
