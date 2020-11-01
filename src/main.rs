mod rot;
use clap::{load_yaml, App, ArgMatches};
use rot::{decryptor::FileDecryptor, encryptor::FileEncryptor};
use std::path::Path;

fn main() {
  // Disclaimer
  println!("This is merely a toy. ROT13 encryption does not provide any cryptographic security.");

  let yaml = load_yaml!("cli.yaml");
  let matches = App::from_yaml(yaml).get_matches();

  // Encrypt branch
  if let Some(matches) = matches.subcommand_matches("encrypt") {
    execute_with_args(matches, |path, outdir, shift, filename| {
      let file_encryptor = FileEncryptor::new(path, shift);
      file_encryptor.encrypt_file(outdir, filename);
    });
  }

  // Decrypt branch
  if let Some(matches) = matches.subcommand_matches("decrypt") {
    execute_with_args(matches, |path, outdir, shift, filename| {
      let file_decryptor = FileDecryptor::new(path, shift);
      file_decryptor.decrypt_file(outdir, filename);
    })
  }
}

// This function is really just to prevent duplicated logic in the encrypt and decrypt branch.
fn execute_with_args<T>(matches: &ArgMatches, task: T)
where
  T: FnOnce(&Path, &str, u8, Option<&str>) -> (), // Ensure the closure T can only be called once
{
  // We can safely unwrap here because path is a required argument.
  let path_str = matches.value_of("path").unwrap();

  // Shift validation aka how many bytes to rotate by during encryption / decryption.
  let shift_str = matches.value_of("shift").unwrap();
  let shift: u8;

  match shift_str.parse::<u8>() {
    Ok(res) => {
      shift = res;
    },
    Err(_) => panic!("Invalid shift value provided. Perhaps you entered a negative value or a value too large. Exiting...")
  }

  let outdir = matches.value_of("outdir").unwrap_or("");
  let filename = matches.value_of("filename");
  let path = Path::new(path_str);
  // Exit function if the path is not valid.
  if !path.is_file() {
    println!("error: The file you're pointing me to doesn't seem to exist :(");
    return;
  }

  // Finally after gathering and validating the args, we execute whatever task is appropriate.
  task(path, outdir, shift, filename);
}
