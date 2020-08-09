use super::{
  constants::{MAX, MIN},
  traits::ByteRotator,
  utils::get_extension,
};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

/// FileEncoder has fields for a path (to a file) and rot_by which signifies by how many bytes
/// it should shift each original byte.
pub struct FileEncryptor<'p> {
  // Path to a file.
  pub path: &'p Path,
  // The number of bytes to rotate by
  pub rot_by: u8,
  min: u8,
  max: u8,
}

impl<'p> FileEncryptor<'p> {
  pub fn new(path: &'p Path, rot_by: u8) -> Self {
    Self {
      path,
      rot_by,
      min: MIN,
      max: MAX,
    }
  }

  /// encrypt_file takes an outdir parameter and optionally a filename parameter.
  /// outdir determines the directory at which the encrypted file will be created at. Provide
  /// empty string for current directory.
  /// filename determines the name of the file. Provide `None` for a default of `"encrypted"`.
  /// # Example
  /// ```
  /// let path = Path::new("test.txt");
  /// let file_encryptor = FileEncryptor::new(path, 13);
  /// // Outputs the file to a relative path.
  /// file_encryptor.encrypt_file("../", Some("rot13"));
  /// assert!(Path::new("../rot13.txt").is_valid());
  /// ```
  pub fn encrypt_file(&self, outdir: &str, filename: Option<&str>) {
    let filename = filename.unwrap_or("encrypted");

    match File::open(self.path) {
      Ok(file) => {
        let buf = BufReader::new(file);
        let mut bytes_vec = vec![];

        for b in buf.bytes() {
          bytes_vec.push(self.rotate_byte(b.unwrap()));
        }

        let outpath = format!("{}{}.{}", outdir, filename, get_extension(self.path));
        // shadow outpath into a string slice because we'll be moving it around.
        let outpath = outpath.as_str();
        let new_file = File::create(outpath);
        match new_file {
          Ok(mut file) => {
            let buf = bytes_vec.as_slice();

            match file.write_all(buf) {
              Ok(()) => println!("Successfully wrote the encrypted file into {}", outpath),
              Err(_) => println!("Error while making the encrypted file"),
            }
          }
          Err(err) => println!("Couldn't create encrypted file due to error: {:?}", err),
        }
      }
      Err(e) => println!("error: {:?}", e),
    }
  }
}

impl ByteRotator for FileEncryptor<'_> {
  /// Takes in a byte and rotates it by self.rot_by.
  fn rotate_byte(&self, b: u8) -> u8 {
    if (self.min..(self.max + 1)).contains(&b) {
      let mut rotated_byte = b + self.rot_by;

      // We want to wrap to the beginning if the rotated byte overflows our max
      if rotated_byte > self.max {
        rotated_byte = (rotated_byte - self.max) + (self.min - 1);
      }

      return rotated_byte;
    }

    b
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::{BufReader, Error, Read};
  use std::path::Path;

  #[test]
  fn it_encrypts_file() {
    const ROT_BY: u8 = 13;
    let fe = FileEncryptor::new(Path::new("test.txt"), ROT_BY);
    fe.encrypt_file("", None);

    match File::open("encrypted.txt") {
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
        assert_eq!(result_str, String::from("Uryy|9 d|!yq"));
      }
      Err(_) => panic!("Failed to open encrypted file"),
    }
  }
}
