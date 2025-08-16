pub fn dp<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    Some(s.strip_prefix(prefix)?)
}

pub use self::dp as delete_prefix;