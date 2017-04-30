use std::fmt;
use std::mem;
use std::ops::Deref;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NulError {
    InteriorNul(usize),
    NotNulTerminated,
}

impl fmt::Display for NulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NulError::InteriorNul(pos) =>
                write!(f, "data provided contains an interior nul at byte pos {}", pos),
            NulError::NotNulTerminated =>
                f.write_str("data provided is not nul terminated"),
        }
    }
}

pub struct NulTerminatedStr(str);

impl NulTerminatedStr {
    pub fn from_str_with_nul(s: &str) -> Result<&NulTerminatedStr, NulError> {
        let nul_pos = s.bytes().position(|b| b == 0);
        nul_pos.ok_or(NulError::NotNulTerminated).and_then(|i| {
            // The first (and only) nul must be at the last index
            if i == s.len() - 1 {
                Ok(unsafe { mem::transmute(s) })
            } else {
                Err(NulError::InteriorNul(i))
            }
        })
    }

    pub fn as_str_with_nul(&self) -> &str {
        &self.0
    }
}

impl Deref for NulTerminatedStr {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0[..self.0.len() - 1]
    }
}

impl AsRef<str> for NulTerminatedStr {
    fn as_ref(&self) -> &str {
        &**self
    }
}

impl fmt::Debug for NulTerminatedStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str_with_nul(), f)
    }
}

impl fmt::Display for NulTerminatedStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[macro_export]
macro_rules! ntstr {
    ($e:expr) => (
        match $crate::NulTerminatedStr::from_str_with_nul(concat!($e, "\0")) {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::{NulTerminatedStr, NulError};

    #[test]
    fn test() {
        let s = ntstr!("foo");
        assert_eq!(&**s, "foo");
        assert_eq!(s.len(), 3);
        assert_eq!(&s[0..2], "fo");
        assert_eq!(s.as_str_with_nul(), "foo\0");
    }

    #[test]
    fn test_err() {
        assert_eq!(NulTerminatedStr::from_str_with_nul("foo").unwrap_err(),
            NulError::NotNulTerminated);
        assert_eq!(NulTerminatedStr::from_str_with_nul("fo\0o").unwrap_err(),
            NulError::InteriorNul(2));
        assert_eq!(NulTerminatedStr::from_str_with_nul("fo\0o\0").unwrap_err(),
            NulError::InteriorNul(2));
    }
}
