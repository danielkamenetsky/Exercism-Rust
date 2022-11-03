use std::collections::HashSet;

// Note: When a function accepts multiple references, they’re each given their own lifetimes 
// We know that the returned reference must be one of the references we received as an input argument, but we don’t know which one.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    // Convert word to lowercase
    let lc_word = word.clone().to_lowercase();
    // Need to sort the word in alphabetical order so that we can match its anagrams
    // To be able to sort it we will need to collect the word into a vector
    // chars() is an iterator
    let mut word_vec = lc_word.chars().collect::<Vec<_>>();
    word_vec.sort_unstable();

    // Next we go through each anagram, if it is the same as the input move on,
    // If not sort and compare. If sorted is equal add to the list

    let mut anagrams = HashSet::new();
    // for possible anagram in the list of anagrams
    for &anagram in possible_anagrams {
        // convert possible anagram to lower case and store
        let lc_anagram = anagram.to_lowercase();
        //if it does not equal the input, sort and check if its an anagram
        if lc_anagram != lc_word {
            // collect into a vector and sort
            let mut another_vec = lc_anagram.chars().collect::<Vec<_>>();
            another_vec.sort_unstable();
            // if it is an anagram insert it into the hashset 
            if another_vec == word_vec {
                anagrams.insert(anagram);
            }
        }
    }
        return anagrams
}

