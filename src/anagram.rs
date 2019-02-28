use hashbrown::HashMap;
use smallvec::{ SmallVec, smallvec };

fn letter_indexes(word: &str, char_count: &usize) -> HashMap<char, usize> {
    let mut letter_indexes = HashMap::with_capacity(*char_count);
    for (idx, letter) in word.to_lowercase().chars().enumerate() {
        letter_indexes.insert(letter, idx);
    }
    letter_indexes
}


fn letter_counts(word: &str, char_count: &usize, letter_indexes: &HashMap<char, usize>) -> SmallVec<[i32; 32]> {
    //TODO wastes a bit of space as currently also includes all not-unique chars
    let mut counts = smallvec![0; *char_count];

    // add first word letters to letter_counts
    for letter in word.to_lowercase().chars() {
        let idx = letter_indexes.get(&letter).unwrap();
        counts[*idx] += 1;
    }
    counts
}

// Calculates reused data. The count of every char in thre word + letter indexes for quick referencing
pub fn precalc_letter_data(word: &str) -> (HashMap<char, usize>, SmallVec<[i32; 32]>) {
    let char_count = word.chars().count();
    let letter_indexes = letter_indexes(word, &char_count);
    let letter_counts = letter_counts(word, &char_count, &letter_indexes);
    (letter_indexes, letter_counts)
}

#[inline(always)]
pub fn is_anagram(candidate: &str, letter_counts: &SmallVec<[i32; 32]>, letter_indexes: &HashMap<char, usize>) -> bool {
    // TODO try packed_simd stuff here?
    let mut counts = letter_counts.clone();
    // subtract candidate letters from word ... sum should be zero
    for letter in candidate.to_lowercase().chars() {
        if !letter_indexes.contains_key(&letter) {
            return false;
        }
        let idx = letter_indexes.get(&letter).unwrap();
        let count = counts[*idx];
        if count == 0 {
            return false;
        }
        counts[*idx] -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // as we optimise some work on the higer level, use this helper function to simplify testing
    fn is_anagram(word: &str, candidate: &str) -> bool {
        let (letter_indexes, letter_counts) = precalc_letter_data(word);
        word.len() == candidate.len() && super::is_anagram(candidate,  &letter_counts, &letter_indexes)
    }

    #[test]
    fn it_validates_simple_anagrams() {
        assert_eq!(is_anagram("foo", "bar"), false);
        assert_eq!(is_anagram("foo", "foo"), true);
        assert_eq!(is_anagram("foo", "fo1"), false);
        assert_eq!(is_anagram("foo", "ofo"), true);
    }

    #[test]
    fn it_works_with_spaces() {
        assert_eq!(is_anagram("foo", " ofo"), false);
        assert_eq!(is_anagram("foo ", " ofo"), true);
        assert_eq!(is_anagram("  ssdiaiisMneiga", "Mida iganes siis", ), true);
        assert_eq!(is_anagram("aGu isAEEtall", "Augeiase tall"), true);
    }

    #[test]
    fn it_is_case_insensitive() {
        assert_eq!(is_anagram("fOO", "oOF"), true);
    }
}