use std::env;
use std::time::{Instant};

mod anagram;
mod naive_finder;
mod parallel_finder;

#[allow(unused_imports)]
use naive_finder::find_anagrams;
use parallel_finder::find_anagrams_parallel;

fn parse_args(args: &[String]) -> (&str, &str) {
    let path = &args[1];
    let word = &args[2];

    assert!(!path.is_empty(), "fires argument expected (a path for the dictionary), but it was empty");
    assert!(!word.is_empty(), "second argument expected (word to search anagrams for), but it was empty");
    (word, path)
}

fn print_result(anagrams: Vec<String>, start: Instant) {
    let elapsed = start.elapsed();
    let elapsed_micros = elapsed.as_secs() as u32 * 1000000 + elapsed.subsec_micros();

    print!("{}", elapsed_micros);
    if !anagrams.is_empty() {
        print!(",");
    }
    println!("{}", anagrams.join(","));
}

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let (word, path) = parse_args(&args);

    // println!("Finding anagrams for '{}'", word);
    // println!("Full path: {}", path);

    // let anagrams = find_anagrams(word, path).unwrap();
    let anagrams = find_anagrams_parallel(word, path);

    print_result(anagrams, start);
}
