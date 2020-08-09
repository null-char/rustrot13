use super::{
  constants::{MAX, MIN},
  traits::ByteRotator,
  utils::get_extension,
};
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

  // TODO: Implement decrypt_file method on FileDecryptor.
  pub fn decrypt_file(outdir: &str, filename: &str) {}
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
