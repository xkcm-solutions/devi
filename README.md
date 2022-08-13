# devi

**( Work In Progress )**

Development CLI tool containing useful commands/tools for various software development. Extensible, based on modules/plugins, built in Rust. 

## Example usage (concept)
Running local FTP server:
```sh
devi ftp-server run -c admin:admin
```
Running local PostgreSQL server:
```sh
devi postgres run -c root:root
```

Converting value to hex:
```sh
devi hex convert-dec 255   # 0xFF
devi hex convert-bin 1001  # 0x9
devi hex convert 0d255     # 0xFF
```

Encrypting/decrypting data with AES:
```sh
devi aes encrypt -k <key> <data>    # <cipher>
devi aes decrypt -k <key> <cipher>  # <data>
```

## Installation

Clone the repo and build the binary with
```
cargo build --release
```
Then run the `./install.sh` script with sudo, the script copies the binary into the `/usr/bin` path (if this directory is not in your $PATH, feel free to modify the script)
```
sudo ./install.sh
```

## Working features

Right now there is only a basic functionality for running local *PostgreSQL* server in a Docker container
```
devi postgres --help
```
