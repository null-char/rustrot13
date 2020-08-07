use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub const MAX: u8 = 126;
pub const MIN: u8 = 33;

// FileEncoder has fields for a path (to a file) and rot_by which signifies by how many bytes
// it should shift each original byte.
pub struct FileEncryptor<'p> {
  // Path to a file.
  pub path: &'p Path,
  // The number of bytes to rotate by
  pub rot_by: u8,
  min: u8,
  max: u8,
}

impl<'p> FileEncryptor<'p> {
  pub fn new(path: &'static Path, rot_by: u8) -> Self {
    Self {
      path,
      rot_by,
      min: MIN,
      max: MAX,
    }
  }

  pub fn encrypt_file(&self) {
    match File::open(self.path) {
      Ok(file) => {
        let buf = BufReader::new(file);
        let mut bytes_vec = vec![];

        for b in buf.bytes() {
          bytes_vec.push(self.rotate_byte(b.unwrap()));
        }

        let new_file = File::create("encrypted.txt");
        match new_file {
          // Borrow as mutable since we'll be writing to it.
          Ok(mut file) => {
            let buf = bytes_vec.as_slice();

            match file.write_all(buf) {
              Ok(()) => println!("Successfully wrote the encrypted file into encrypted.txt"),
              Err(_) => println!("Error while making the encrypted file"),
            }
          }
          Err(err) => println!("Couldn't create encrypted file due to error: {:?}", err),
        }
      }
      Err(e) => println!("error: {:?}", e),
    }
  }

  // Takes in a byte and rotates it by self.rot_by.
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
    fe.encrypt_file();

    match File::open("encrypted.txt") {
      Ok(file) => {
        let buf = BufReader::new(file);
        let results = buf.bytes().collect::<Vec<Result<u8, Error>>>();
        let mut bytes_vec: Vec<u8> = vec![];

        for byte in results {
          match byte {
            Ok(b) => bytes_vec.push(b),
            Err(err) => panic!(
              "Couldn't read all the bytes from encrypted file due to err: {:?}",
              err
            ),
          }
        }

        let result_str = String::from_utf8(bytes_vec).unwrap();
        assert_eq!(result_str, String::from("Uryy|9 d|!yq"));
      }
      Err(_) => panic!("Failed to open encrypted file"),
    }
  }
}
