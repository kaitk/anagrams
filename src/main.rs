use std::env;
use std::fs::{ read_to_string, File };
use std::io::{BufRead, BufReader, Result};
use std::time::{Instant};

use hashbrown::HashMap;
use rayon::prelude::*;

fn letter_counts(word: &str, char_count: &usize, letter_indexes: &HashMap<char, usize>) -> Vec<i32> {
    //TODO wastes space, currently also includes all not-unique chars
    let mut counts = vec![0; *char_count];

    // add first word letters to letter_counts
    for letter in word.to_lowercase().chars() {
        let idx = letter_indexes.get(&letter).unwrap();
        counts[*idx] += 1;
    }
    counts
}

#[allow(dead_code)]
fn is_anagram(candidate: &str, letter_counts: &Vec<i32>, letter_indexes: &HashMap<char, usize>) -> bool {
    // TODO try packed_simd stuff here
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

    counts.iter().all(|&count| count == 0)
    // counts.iter().sum::<i32>() == 0
}

// Calculates reused data. The count of every char in thre word + letter indexes for quick referencing
fn precalc_letter_data(word: &str) -> (HashMap<char, usize>, Vec<i32>) {
    let char_count = word.chars().count();
    let mut letter_indexes = HashMap::with_capacity(char_count);
    for (idx, letter) in word.to_lowercase().chars().enumerate() {
        letter_indexes.insert(letter, idx);
    }
    let word_counts = letter_counts(word, &char_count, &letter_indexes);
    (letter_indexes, word_counts)
}


#[allow(dead_code)]
fn find_anagrams(word: &str, path: &str) -> Result<Vec<String>> {
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

//TODO Can be memory Hungry, as loads the entire file into memory at once
#[allow(dead_code)]
fn find_anagrams_parallel(word: &str, path: &str) -> Vec<String> {

    let (letter_indexes, letter_counts) = precalc_letter_data(word);

    let contents = read_to_string(path)
        .expect("Something went wrong reading the file");

    // let mut results: Vec<String> = Vec::new();
    contents
        .par_lines()
        .filter(|candidate|  {
            candidate.len() == word.len() && is_anagram(candidate, &letter_counts,  &letter_indexes)
        })
        .map(|candidate| candidate.to_string().clone())
        .collect()
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let path = &args[1];
    let word = &args[2];

    assert!(!path.is_empty(), "fires argument expected (a path for the dictionary), but it was empty");
    assert!(!word.is_empty(), "second argument expected (word to search anagrams for), but it was empty");
    (word, path)
}

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let (word, path) = parse_args(&args);

    println!("Finding anagrams for '{}'", word);
    println!("Full path: {}", path);

    // let anagrams = find_anagrams(word, path).unwrap();
    let anagrams = find_anagrams_parallel(word, path);

    let elapsed = now.elapsed();
    let elapsed_micros = elapsed.as_secs() as u32 * 1000000 + elapsed.subsec_micros();

    // println!("Benchmark took {} microseconds", elapsed_micros);
    // println!("Anagrams: {:?} ", anagrams.join(","));
    println!("{},{}", elapsed_micros, anagrams.join(","));

}