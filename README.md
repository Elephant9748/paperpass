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
  -list                         Print list of pgpkey

Command:
  init                          Set init config generate toml
      -c <YOUR/CONFIG/PATH>     Where config saved
      -s <YOUR/DATA/STORE>      Where data store
      -pk                       Encrypt data with apgp key by uid (name of key)
  show -s <YOUR/DATA/STORE>     Show secret
  totp <YOUR/DATA/STORE>        Display totp every 30
  totp -c <YOUR/DATA/STORE>     Copy totp into clipboard
  ls <YOUR/DATA/STORE>  List of secrets
```

### format password with otpauth
```
[password or data to encrypt]
---
[otpauth url]
```
