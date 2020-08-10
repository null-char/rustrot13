// Utility functions for rot module to do its stuff.
use std::ffi::OsStr;
use std::path::Path;

/// Gives back the output directory path to the new transformed file.
/// `instance_path` signifies the original path to the file that was encrypted / decrypted.
pub fn parse_to_path(outdir: &str, filename: &str, instance_path: &Path) -> String {
  let outpath = format!("{}{}.{}", outdir, filename, get_extension(instance_path));
  outpath
}

/// Returns the extension of a path as a string slice.
/// # Examples
/// ```
/// let p = Path::new("some_file.txt");
/// let ext = get_extension(p);
/// println!("Extension of some_file is {}", ext);
/// ```
fn get_extension(p: &Path) -> &str {
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

  #[test]
  fn it_parses_to_path() {
    let instance_path = Path::new("how to make pineapple on pizza (eww).txt");
    // This translates to a directory path of: "f/(TOP SECRET) recipes/forbidden recipe.txt"
    let outdir = "f/(TOP\\ SECRET)\\ recipes/";
    let filename = "forbidden\\ recipe";
    let output = parse_to_path(outdir, filename, instance_path);
    assert_eq!(output, "f/(TOP\\ SECRET)\\ recipes/forbidden\\ recipe.txt")
  }
}
