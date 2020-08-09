# rustrot13
ROT13 file encryption using Rust.

## Disclaimer
In case it isn't obvious already, ROT13 file encryption is basically equivalent to showing the answers to a quiz upside down as it
provides virtually no cryptographic security. This is a CLI application that I'm working on to learn more about Rust.
The code is extremely basic and is mostly just file I/O.

## How to Use
If you want to, you can compile a release binary application by executing:  
```cargo build --release```  
The binary can be found in target/release. Try executing the app with --help in your terminal to get a list of possible commands and arguments.  
Another alternative is to simply run main.rs directly using cargo.  
```cargo run```  

### Encryption
Currently, only encryption is supported. Simply run the command `rot13 encrypt [OPTIONS] <PATH_TO_FILE>` to encrypt something using ROT13. For help on the encrypt
subcommand you can run `rot13 encrypt --help`.
