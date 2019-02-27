use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::time::{Instant};

use hashbrown::HashMap;
// use smallvec::{smallvec, SmallVec};

#[allow(dead_code)]
const LETTERS_LEN: usize = 65;
#[allow(dead_code)]
const LETTERS: [char; LETTERS_LEN] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'Š', 'Z', 'Ž', 'T', 'U', 'V', 'W', 'Õ', 'Ä', 'Ö', 'Ü', 'X', 'Y',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 'š', 'z', 'ž', 't', 'u', 'v', 'w', 'õ', 'ä', 'ö', 'ü', 'x', 'y',
    ' '
];


#[allow(dead_code)]
fn calc_total(path: &str) -> Result<()> {
    let now = Instant::now();

    let mut total = 0;

    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        // let string = &line?.to_string();
        // total += &line?.len();
        total += &line?.chars().count();
    }

    let elapsed = now.elapsed();

    println!("Benchmark took {} microseconds", elapsed.subsec_micros());
    println!("Total: {} ", total);
    Ok(())
}

#[allow(dead_code)]
fn find_anagrams(word: &str, path: &str) -> Result<()> {
    let now = Instant::now();

    // initialize index-map
    let mut letter_indexes = HashMap::new();
    for (idx, letter) in word.chars().enumerate() {
        letter_indexes.insert(letter, idx);
    }
    let cached_counts = get_counts(word, &letter_indexes);

    // let mut results: SmallVec<[&str; 100]> = SmallVec::with_capacity(100);
    let mut results: Vec<String> = Vec::with_capacity(100);


    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let string = &line?;
        // if string.len() == word.len() && is_anagram(word, string, &letter_indexes) {
        if string.len() == word.len() && is_anagram2(cached_counts, string, &letter_indexes) {
            results.push(string.clone());
        }

    }

    let elapsed = now.elapsed();

    println!("Benchmark took {} microseconds", elapsed.subsec_micros());
    println!("Anagrams: {:?} ", results);
    Ok(())
}

fn get_counts(word: &str, letter_indexes: &HashMap<char, usize>) -> [i32; LETTERS_LEN] {
    //TODO check, can overflow
    let mut counts = [0; LETTERS_LEN];
    // add first word letters
    for letter in word.chars() {
        let idx = letter_indexes.get(&letter).unwrap();
        counts[*idx] += 1;
    }
    counts
}

fn is_anagram(first: &str, second: &str, letter_indexes: &HashMap<char, usize>) -> bool {
    let mut counts = get_counts(first, letter_indexes);

    // subtract other word letters
    for letter in second.chars() {
        //TODO seems like a slower way of finding these
        match letter_indexes.get(&letter) {
            Some(idx) => {
                let count = counts[*idx];
                if count == 0 {
                    return false;
                }
                counts[*idx] -= 1;
            },
            None => return false
        }
    }

    // println!("Sum for {}, {} is {}", first, second, counts.iter().sum::<i32>());
    counts.iter().sum::<i32>() == 0
}

fn is_anagram2(cached_counts: [i32; LETTERS_LEN], second: &str, letter_indexes: &HashMap<char, usize>) -> bool {
    let mut counts = cached_counts.clone();

    // subtract other word letters
    for letter in second.chars() {
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

    counts.iter().sum::<i32>() == 0
}





fn main() -> Result<()> {
    let path = "/Users/kaitkasak/projects/anagrams/".to_string();
    let file_name = "lemmad.txt";
    // let file_name = "short.txt";
    let word = "Augeiase tall";

    // return calc_total(&(path + &file_name));
    return find_anagrams(word, &(path + &file_name));
}