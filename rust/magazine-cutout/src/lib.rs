// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_words = HashMap::new();
    let mut note_words = HashMap::new();
    for m in magazine {
        *mag_words.entry(*m).or_insert(0) += 1;
    }
    for n in note {
        *note_words.entry(*n).or_insert(0) += 1;
    }
    for (note_word, note_count) in &note_words {
        if !mag_words.contains_key(note_word) {
            return false;
        }
        let mag_word_count = mag_words.get(note_word).unwrap();
        if note_count > mag_word_count {
            return false;
        }
    }
    return true;
}
