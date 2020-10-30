use std::path::{Path, PathBuf};

/// compress `..` and `.`
/// works on absolute path
pub fn compress<P: AsRef<Path>>(from: P) -> PathBuf {
    let from = from.as_ref();
    let mut path = PathBuf::new();

    for i in from.into_iter() {
        match &*i.to_string_lossy() {
            "." => {}
            ".." => {
                if let Some(parent) = path.parent() {
                    path = parent.to_path_buf();
                }
            }
            _ => {
                path = path.join(i);
            }
        }
    }

    path
}

#[cfg(test)]
mod path_test {
    use super::*;

    #[test]
    fn pathbuf_compress_test() {
        assert_eq!(compress("/hello/../world/./"), PathBuf::from("/world"));
        assert_eq!(compress("/../world/./"), PathBuf::from("/world"));
    }
}
