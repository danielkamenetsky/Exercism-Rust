// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;


pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    // creating a hashmap for the note
    let mut note_hashmap: HashMap<&&str, usize> = HashMap::new(); 
    //iterating over the note vector
    for word in note {
        let count = note_hashmap.entry(word).or_insert(0);
        *count +=1;
    }
    //viewing what the hashmap looks like <not required but recommended for beginners to understand what is going on at this point>
    for (word, count) in &note_hashmap {
        println!("{}, {}", word, count);
    }
    println!("---------------");
    let mut magazine_hashmap: HashMap<&&str, usize> = HashMap::new(); 
    
    for word in magazine {
        let count = magazine_hashmap.entry(word).or_insert(0);
        *count +=1;
    }
    for (word, count) in &magazine_hashmap {
        println!("{}, {}", word, count);
    }
    
    //creating a bool to determine if words are the same
    let mut compare_maps: bool = false;
    // looping over keys of hash map
    for i in note_hashmap.keys() {
        // check if magazine doesn't contain the string
        if !magazine_hashmap.contains_key(i) {
            compare_maps = false;
            break;
            }
        // checking if word appears at least as many times in magazine as it does in the note
        else if magazine_hashmap.get(i) < note_hashmap.get(i) {
            compare_maps = false;
            break;
        }

        else {
        compare_maps = true;
        }
    }
   compare_maps    
   
}