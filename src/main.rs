mod rot;
use clap::{load_yaml, App};
use rot::encryptor::FileEncryptor;
use std::path::Path;

fn main() {
  // Disclaimer
  println!("This is merely a toy. ROT13 encryption does not provide any cryptographic security.");

  let yaml = load_yaml!("cli.yaml");
  let matches = App::from_yaml(yaml).get_matches();

  // Encrypt branch
  if let Some(matches) = matches.subcommand_matches("encrypt") {
    // We can safely unwrap here because path is a required argument.
    let path_str = matches.value_of("path").unwrap();
    let outdir = matches.value_of("out_dir").unwrap_or("");
    let filename = matches.value_of("filename");
    let path = Path::new(path_str);

    // Exit main if the path is not valid.
    if !path.is_file() {
      println!("error: The file you're pointing me to doesn't seem to exist :(");
      return;
    }

    let file_encryptor = FileEncryptor::new(path, 13);
    file_encryptor.encrypt_file(outdir, filename);
  }
}
