use super::{
  constants::{MAX, MIN},
  traits::ByteRotator,
  utils::parse_to_path,
};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub struct FileDecryptor<'p> {
  pub path: &'p Path,
  pub rot_by: u8,
  min: u8,
  max: u8,
}

impl<'p> FileDecryptor<'p> {
  pub fn new(path: &'p Path, rot_by: u8) -> Self {
    Self {
      path,
      rot_by,
      min: MIN,
      max: MAX,
    }
  }

  pub fn decrypt_file(&self, outdir: &str, filename: Option<&str>) {
    let filename = filename.unwrap_or("decrypted");

    match File::open(self.path) {
      Ok(file) => {
        let buf = BufReader::new(file);
        let mut decrypted_bytes: Vec<u8> = vec![];

        for byte in buf.bytes() {
          // Stop execution if reading one of the bytes failed.
          if let Err(_) = byte {
            println!("error: Reading bytes from file for decryption failed");
            return;
          }

          let rotated_byte = self.rotate_byte(byte.unwrap());
          decrypted_bytes.push(rotated_byte);
        }

        // Determine the directory at which to create the output file
        let outpath = parse_to_path(outdir, filename, self.path);
        let outpath = outpath.as_str();

        match File::create(outpath) {
          Ok(mut file) => {
            let buf = decrypted_bytes.as_slice();
            match file.write_all(buf) {
              Ok(()) => println!("Successfully wrote the decrypted file to {}", outpath),
              Err(_) => println!("error: Writing to decrypted file failed"),
            }
          }
          Err(err) => println!(
            "error: Creation of decrypted file failed due to err: {:?}",
            err
          ),
        }
      }
      Err(_) => println!("error: File for decryption couldn't be opened"),
    }
  }
}

impl ByteRotator for FileDecryptor<'_> {
  fn rotate_byte(&self, byte: u8) -> u8 {
    if (self.min..(self.max + 1)).contains(&byte) {
      let mut new_byte = byte - self.rot_by;

      // Wrap back to max.
      if new_byte < self.min {
        new_byte = (self.max + 1) - (self.min - new_byte);
      }

      return new_byte;
    }

    byte
  }
}

#[cfg(test)]
mod tests {
  use super::{super::constants::ROT_BY, *};
  use std::fs::File;
  use std::io::{BufReader, Error, Read};
  use std::path::Path;

  #[test]
  fn it_decrypts_file() {
    let fd = FileDecryptor::new(Path::new("test2.txt"), ROT_BY);
    fd.decrypt_file("", None);

    match File::open("decrypted.txt") {
      Ok(file) => {
        let buf = BufReader::new(file);
        let results = buf.bytes().collect::<Vec<Result<u8, Error>>>();
        let bytes_vec: Vec<u8> = results
          .into_iter()
          .map(|b| match b {
            Ok(b) => return b,
            Err(err) => panic!("Error mapping results: {:?}", err),
          })
          .collect();

        let result_str = String::from_utf8(bytes_vec).unwrap();
        assert_eq!(result_str, String::from("This is for the decryption test."));
      }
      Err(_) => panic!("Failed to open decrypted file"),
    }
  }
}
