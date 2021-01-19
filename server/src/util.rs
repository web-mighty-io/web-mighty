use std::env;
use std::path::{Path, PathBuf};

/// This compresses the input path.
/// `Path::join` just pushes second path to first one.
/// Therefore joining `/hello/..` and `world` results to `/hello/../world`
///
/// This function compresses the result of joined path.
/// `.` will be removed and `..` will make the path to go parent.
///
/// If input is absolute path, `/..` will be ignored.
/// If input is relative path, `..` in front will be remain same.
///
/// # Examples
///
/// ```
/// use server::prelude::compress;
/// use std::path::PathBuf;
///
/// assert_eq!(compress("/../world/./"), PathBuf::from("/world"));
/// assert_eq!(compress("hello/../../world"), PathBuf::from("../world"));
/// ```
pub fn compress<P: AsRef<Path>>(from: P) -> PathBuf {
    let from = from.as_ref();
    let mut path = PathBuf::new();
    let is_absolute = from.is_absolute();

    for i in from.iter() {
        match &*i.to_string_lossy() {
            "." => {}
            ".." => {
                if let Some(parent) = path.parent() {
                    path = parent.to_path_buf();
                } else if !is_absolute {
                    path = path.join("..")
                }
            }
            _ => {
                path = path.join(i);
            }
        }
    }

    path
}

/// Changes the path to absolute path.
/// If input path is relative, it concat with current directory path.
/// If input path is absolute, it returns input.
///
/// # Examples
///
/// ```no_run
/// use server::prelude::to_absolute_path;
/// use std::env;
/// use std::path::PathBuf;
///
/// env::set_current_dir("/hello");
/// assert_eq!(to_absolute_path("world"), PathBuf::from("/hello/world"));
/// assert_eq!(to_absolute_path("/world"), PathBuf::from("/world"));
/// ```
pub fn to_absolute_path<P: AsRef<Path>>(from: P) -> PathBuf {
    let from = from.as_ref();
    if from.is_relative() {
        compress(env::current_dir().unwrap().join(from))
    } else {
        from.to_path_buf()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compress_test() {
        assert_eq!(compress("/hello/../world/./"), PathBuf::from("/world"));
        assert_eq!(compress("/../world/./"), PathBuf::from("/world"));
        assert_eq!(compress("hello/../../world"), PathBuf::from("../world"));
    }

    #[test]
    fn to_absolute_path_test() {
        env::set_current_dir("/hello").unwrap();
        assert_eq!(to_absolute_path("world"), PathBuf::from("/hello/world"));
        assert_eq!(to_absolute_path("/world"), PathBuf::from("/world"));
    }
}
