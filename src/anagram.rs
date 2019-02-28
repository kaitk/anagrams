use hashbrown::HashMap;

fn letter_indexes(word: &str, char_count: &usize) -> HashMap<char, usize> {
    let mut letter_indexes = HashMap::with_capacity(*char_count);
    for (idx, letter) in word.to_lowercase().chars().enumerate() {
        letter_indexes.insert(letter, idx);
    }
    letter_indexes
}


fn letter_counts(word: &str, char_count: &usize, letter_indexes: &HashMap<char, usize>) -> Vec<i32> {
    //TODO wastes space as currently also includes all not-unique chars
    let mut counts = vec![0; *char_count];

    // add first word letters to letter_counts
    for letter in word.to_lowercase().chars() {
        let idx = letter_indexes.get(&letter).unwrap();
        counts[*idx] += 1;
    }
    counts
}

// Calculates reused data. The count of every char in thre word + letter indexes for quick referencing
pub fn precalc_letter_data(word: &str) -> (HashMap<char, usize>, Vec<i32>) {
    let char_count = word.chars().count();
    let letter_indexes = letter_indexes(word, &char_count);
    let letter_counts = letter_counts(word, &char_count, &letter_indexes);
    (letter_indexes, letter_counts)
}

#[inline(always)]
pub fn is_anagram(candidate: &str, letter_counts: &Vec<i32>, letter_indexes: &HashMap<char, usize>) -> bool {
    // TODO try packed_simd stuff here?
    let mut counts: Vec<i32> = letter_counts.clone();
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