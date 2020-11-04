# rustrot13
ROT13 file encryption using Rust.

## Disclaimer
In case it isn't obvious already, ROT13 file encryption is basically equivalent to showing the answers to a quiz upside down as it
provides virtually no cryptographic security. This is a CLI application that I'm working on to learn more about Rust.
The code is extremely basic and is mostly just file I/O.

## How to Use
If you want to, you can compile a release binary application by executing: `cargo build --release`.  
The binary can be found in target/release. Try executing the app with --help in your terminal to get a list of possible commands and arguments.  
  
Another alternative is to simply run main.rs directly using cargo: ```cargo run```  
  
### Encryption
For "encrypting" a file (really just rotating bytes by +13 or whichever key shift you provide), simply run the command `rot13 encrypt [OPTIONS] <PATH_TO_FILE>` to encrypt something using the provided shift value (by default this is 13). For help on the encrypt subcommand you can run `rot13 encrypt --help`. Additionally, if you'd like to set the shift value to something different you can do so via the `shift` argument.

## Decryption
This simply just involves backshifting the bytes given a key (which is the shift value). Simply run the command `rot13 decrypt [OPTIONS] <PATH_TO_FILE> <SHIFT>` to decrypt something. You'll get back something coherent as long as the key you provided is valid for that particular file. For help on the decrypt subcommand you can run `rot13 decrypt --help`.

# Notes
It should be noted that currently all of the bytes pertaining to ASCII codes 33 - 122 are rotated. This is hardcoded as a constant for now.
