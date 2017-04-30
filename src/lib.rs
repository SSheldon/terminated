use std::mem;
use std::ops::Deref;

pub struct NulError;

pub struct NulTerminatedStr(str);

impl NulTerminatedStr {
    pub fn from_str_with_nul(s: &str) -> Result<&NulTerminatedStr, NulError> {
        let bytes = s.as_bytes();
        if bytes.last() != Some(&0) {
            Err(NulError)
        } else if bytes[..bytes.len()-1].contains(&0) {
            Err(NulError)
        } else {
            Ok(unsafe { mem::transmute(s) })
        }
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

#[macro_export]
macro_rules! ntstr {
    ($e:expr) => (
        match $crate::NulTerminatedStr::from_str_with_nul(concat!($e, "\0")) {
            Ok(s) => s,
            Err(_) => panic!(),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::NulTerminatedStr;

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
        assert!(NulTerminatedStr::from_str_with_nul("foo").is_err());
        assert!(NulTerminatedStr::from_str_with_nul("fo\0o").is_err());
        assert!(NulTerminatedStr::from_str_with_nul("fo\0o\0").is_err());
    }
}
