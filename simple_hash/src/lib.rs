use std::collections::HashMap;

/*
Create a function named word_frequency_counter which will receive a vector of strings (each string being a single word) and return an HashMap with the word as the key and the number of repetitions as the value.

Also create a function named nb_distinct_words which will take a reference to the HashMap and return the number of distinct words present in it.

all the words tested will be lowercase
*/
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map: HashMap<&str, usize>=  HashMap::with_capacity(words.len());
    for &ele in words.iter() {
        *( map.entry(ele).or_insert(0_usize))+=1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}