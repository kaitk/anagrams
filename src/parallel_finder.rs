use std::fs::{ read_to_string };
use rayon::prelude::*;

use crate::anagram::{ is_anagram, precalc_letter_data };

/*
  Notice Can be memory Hungry, as loads the entire file into memory at once.
  No good way to do it differently for now due to limitations in Rayon
*/
pub fn find_anagrams_parallel(word: &str, path: &str) -> Vec<String> {

    let (letter_indexes, letter_counts) = precalc_letter_data(word);

    let contents = read_to_string(path).expect("failed to read file");

    contents
        .par_lines() // <- this is all the parallelization magic
        .filter(|candidate|  {
            candidate.len() == word.len() && is_anagram(candidate, &letter_counts,  &letter_indexes)
        })
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_actual_dictionary() {
        let result = find_anagrams_parallel("aGu isAEEtall", "dicts/lemmad.txt");
        assert_eq!(result, ["Augeiase tall"])
    }

    #[test]
    fn it_finds_multiple() {
        let result = find_anagrams_parallel("aSi", "dicts/lemmad.txt");
        assert_eq!(result, ["ais", "asi", "isa", "sai"])
    }
}