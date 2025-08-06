# paperpass
> [!WARNING]
> not ready to be used 
## Usage
```
Usage: paperpass [COMMAND] [OPTIONS] [ARGS]
       paperpass [GLOBAL OPTIONS] [ARGS]

Options global:
  -c <YOUR/DATA/STORE>          Copy to clipboard
     --time <TIME>              Copy to clipboard with autoclear after amunt of sec TIME=NUMBER
  -h                            Print help
  -v                            Print version
  -list                         Print list of pgpkey

Command:
  init                          Set init config generate toml
      -c <YOUR/CONFIG/PATH>     Where config saved
      -s <YOUR/DATA/STORE>      Where data store
      -pk                       Encrypt data with apgp key by uid (name of key)
  show -s <YOUR/DATA/STORE>     Show secret
  otp -c <YOUR/DATA/STORE>      Display otp every 30
```
