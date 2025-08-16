pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    Some(s.strip_prefix(prefix)?)
}
