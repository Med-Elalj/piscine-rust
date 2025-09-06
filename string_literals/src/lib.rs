pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    if v.len()>= index {
        v.split_at(index)
    } else {
        (v,"")
    }
}

pub fn find(v: &str, pat: char) -> usize {
    if let Some(i)= v.find(pat){
        i
    } else {
        usize::MAX
    }
}