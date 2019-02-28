use std::fs::{ File };
use std::io::{BufRead, BufReader, Result};

use crate::anagram::{ is_anagram, precalc_letter_data };

pub fn find_anagrams(word: &str, path: &str) -> Vec<String> {
    let (letter_indexes, letter_counts) = precalc_letter_data(word);

    let file = File::open(path).expect("failed to read file");

    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|candidate| {
            candidate.len() == word.len() && is_anagram(candidate, &letter_counts,  &letter_indexes)
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_actual_dictionary() {
        assert_eq!(find_anagrams("aGu isAEEtall", "dicts/lemmad.txt").unwrap(), ["Augeiase tall"])
    }

    #[test]
    fn it_finds_multiple() {
        assert_eq!(find_anagrams("aSi", "dicts/lemmad.txt").unwrap(), ["ais", "asi", "isa", "sai"])
    }
}