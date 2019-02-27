use std::fs::{ File };
use std::io::{BufRead, BufReader, Result};

use crate::anagram::{ is_anagram, precalc_letter_data };

#[allow(dead_code)]
pub fn find_anagrams(word: &str, path: &str) -> Result<Vec<String>> {
    let (letter_indexes, letter_counts) = precalc_letter_data(word);

    let mut results: Vec<String> = Vec::new();

    let file = File::open(path)
        .expect("Something went wrong reading the file");

    for line in BufReader::new(file).lines() {
        let candidate = &line?;
        if candidate.len() == word.len() && is_anagram(candidate, &letter_counts,  &letter_indexes) {
            results.push(candidate.clone());
        }

    }
    Ok(results)
}