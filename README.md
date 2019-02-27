# Anagrams
A Quick Take on anagram finding in Rust.

The requirements for the specific task are [discussed here](https://www.helmes.com/careers/challenge/?fbclid=IwAR24MjYoBzK-QNJSMeu_afxYtqJA42h-VmIxaC66b6nJgC16vx-CLq3y_sc).
In essense, make a program, that takes a dictionary and a word as arguments and find all the anagrams of the word in the dictionary

## Usage

**Note:** the program *will crash* if the dictionary is not in UTF-8 encoding and contains non-ASCII characters.

### Executing existing Linux binary
The project includes a linux binary (compiled under ubuntu 18.04) int the `.bin dir`. It also includes the standard "lemmad.txt" file, specified in the requirements.
Searching for anagrams for a word is done with the following command:

**NB!** Included binary only works under linux!

```
bin/anagrams dicts/lemmad.txt asi
> 4587,ais,asi,isa,sai
```
where the first argument is the dictionary and the other one is the word to search for. 

To use a word with spaces as args do the following:
```
bin/anagrams dicts/lemmad.txt "aGu isAEEtall"
> 4674,Augeiase tall
```

The output is the execution time in microseconds and the found anagrams.

### Compiling from source (any platform)

1. install [rust](https://www.rust-lang.org/tools/install):
```
curl https://sh.rustup.rs -sSf | sh
```

2. build the project:
```
cargo build --release
```

3. run the executable: (built into the `./target/release` folder).
```
./target/release/anagrams dicts/lemmad.txt anagramm
```
(or with any other desired parameters)


# About the solution:

## Algorithm
The naive approach would be, to compare the length word to every string in the dictionary. If it matches sort both strings and validate that they are equal. That however is far from optimal as comparison based sorting has a minimal complexitiy of `O(n log n)`.

Instead we could "abuse" the fact that the possible number of characters is limited and use something similar to counting sort `O(n)`.

E.g. We could create an "alphabet vector" for the first string (containing all the possible characters one could encounter in the dictionary). Then:

1. Check that the strings have equal length
2. If true, then traverse the first string adding 1 to the positions of each letter in the vector.
3. Traverse the 2nd string substracting 1 from the vector for each letter in the string. When encountering a zero `return false` immediately as the current letter in the 2nd string has more occurrences than in the 1st one
4. once the entire string is traversed the strngs must be equal, `return true`

Relevant Javascript code:
```javascript
// Example code in javascript for simplicity
function isAnagram(word, candidate) {
  const counts = new Array(letters.length).fill(0);
  // add letters of the search word
  for(let letter of word) {
    counts[indexes[letter]] += 1;
  }
  // subtract the letters of the candidate word
  for(let letter of candidate) {
    const idx = indexes[letter];
    const count = memo[idx];
    if(count === 0) return false 
    counts[idx] -= 1;
  }
  return true;
}
```
### Optimizations
Due to the specifics of the task, we can make some rather obvious "optimizations".
* As we have a fixed "search word" we don't need the entire alphabet, we can just create a vector containing all the symbols of only that word. If candidate word doesn't have one of these, return false immediately.
* We can precalculate the count vector, as it's constantly reused.

That algorithm is closer to:
```javascript
//Example code in javascript for simplicity
function isAnagram(letterCounts, candidate) {
  const counts = [...letterCounts]; //clone

  for(let letter of candidate) {
    const idx = indexes[letter];
    const count = memo[idx];
    if(count === 0) return false 
    counts[idx] -= 1;
  }
  return true;
}
```

This has the algorithm settled. The rest is implementation details and I/O stuff (as this excercise is majorly I/O bound).


# Why RUST?
Rust is a systems programming language that makes [fearless concurrency](https://blog.knoldus.com/how-we-can-do-fearless-concurrency-in-rust/) relatively easy. It's lightweight, has no carbage collection and has modern build-, packaging- and unit-testing tools included out of the box.

On top of that RUST allows pretty simple Thread level Parallelism (via [rayon](https://github.com/rayon-rs/rayon)) and Instruction Level Parallelism (via SIMD libraries, like [numeric-array](https://github.com/novacrazy/numeric-array)) without changing your code.

As the anagram problem is embarrassingly parallel, it probably makes sense to use at least thread-level concurrency. 

For instance: the function for streaming the file and finding anagrams sequencially is the following
```rust
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
```

The only change needed to make to make this run on all the logical processors available (e.g. 12 threads on a modern 6-core i7) is the following:
```rust
pub fn find_anagrams_parallel(word: &str, path: &str) -> Vec<String> {

    let (letter_indexes, letter_counts) = precalc_letter_data(word);

    let contents = read_to_string(path)
        .expect("Something went wrong reading the file");

    contents
        .par_lines() // <- this is all the parallelization magic there is
        .filter(|candidate|  {
            candidate.len() == word.len() && is_anagram(candidate, &letter_counts,  &letter_indexes)
        })
        .map(|candidate| candidate.to_string().clone())
        .collect()
}
```

The only real change is replacing `.iter()` with `.par_iter()` and Rayon does the rest! :)

Although one must note, that the parallel doesn't stream. Rather it loads the entire dictionary to memory in the beginning. This is required for two reasons:
1. The vast majority of time goes to fetching the dictonary. When buffering there is no real benefit from using multiple cores.
2. Rayon [doesn't have a good answer](https://users.rust-lang.org/t/rayon-parallelism-on-the-lines-of-a-text-file/12481) to the problem yet (and probably won't have, until async/await is added to rust).

### Rust specific optimizations:
* Rayon for parallelizing
* HashBrown instead of system hashmaps (uses google's swiss-tables and is considerably more optimized)
* [WIP] numeric-arrays or small-vectors instead of system vectors
* Enabling Link Time Optimizations in Cargol.toml - No real benefit
* mark some methods for inlining - No real benefit
* Tired forcing target-native (to allow SIMD on modern processors) - No real benefit.
* Tried jemalloc instead of system-allocator - reverted, No benefit

## Some benchmarking:
* Reference buffering node version finds anagrams of 'Agu isaeetall' in 
`~37648 microseconds`
* The in-memory parallel rust version finds it in 
`~4471 microseconds`
