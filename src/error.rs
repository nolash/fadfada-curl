use std::fmt;

pub struct NoContentError {}

impl fmt::Display for NoContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(f, format_args!("no content"))
    }
}

impl fmt::Debug for NoContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::write(f, format_args!("no content"))
    }
}
