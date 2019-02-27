# Anagrams
A Quick Take on anagram creation in Rust

## Task:
[Discussed here](https://www.helmes.com/careers/challenge/?fbclid=IwAR24MjYoBzK-QNJSMeu_afxYtqJA42h-VmIxaC66b6nJgC16vx-CLq3y_sc)
In essense, make a program, that takes a dictionary and a word as arguments and find all the anagrams of the word in the dictionary

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
