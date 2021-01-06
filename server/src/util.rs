use crate::dev::*;
use actix::dev::ToEnvelope;
use actix::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

pub fn send<A, B, M>(actor: &A, ctx: &mut A::Context, to: Addr<B>, msg: M) -> Result<M::Result, MailboxError>
where
    A: Actor,
    A::Context: AsyncContext<A>,
    B: Actor,
    M: Message + Send + 'static,
    M::Result: Send,
    B: Handler<M>,
    B::Context: ToEnvelope<B, M>,
{
    let mut x = Err(MailboxError::Closed);
    let r = &mut x as *const Result<M::Result, MailboxError> as *mut Result<M::Result, MailboxError>;
    // SAFETY: referencing `x` is finished inside unsafe code block
    unsafe {
        to.send(msg)
            .into_actor(actor)
            .then(move |res, _, _| {
                *r = res;
                fut::ready(())
            })
            .wait(ctx);
    }
    x
}

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
    fn path_buf_compress_test() {
        assert_eq!(compress("/hello/../world/./"), PathBuf::from("/world"));
        assert_eq!(compress("/../world/./"), PathBuf::from("/world"));

        assert_eq!(compress("hello/../../world"), PathBuf::from("../world"));
    }
}
