// Utility functions for rot module to do its stuff.
use std::ffi::OsStr;
use std::path::Path;

/// Returns the extension of a path as a string slice.
/// # Examples
/// ```
/// let p = Path::new("some_file.txt");
/// let ext = get_extension(p);
/// println!("Extension of some_file is {}", ext);
/// ```
pub fn get_extension(p: &Path) -> &str {
  let ext = p.extension().and_then(OsStr::to_str);

  match ext {
    Some(ext) => return ext,
    // Panic if we never get back an extension. This will most likely be due to incorrect input.
    None => panic!(
      "Couldn't get file extension. Check filename and ensure that it contains an extension."
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_gets_extension() {
    let path = Path::new("some_file.txt");
    let ext = get_extension(path);
    assert_eq!(ext, "txt");
  }
}
