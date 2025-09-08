use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map: HashMap<&str, usize>=  HashMap::with_capacity(words.len());
    for &e in words.iter() {
        *( map.entry(e).or_insert(0_usize))+=1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}