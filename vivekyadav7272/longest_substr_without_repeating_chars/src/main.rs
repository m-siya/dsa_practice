use std::cmp::max;
use std::time::Instant;

macro_rules! new_hash_map {
    ($($key:expr => $val:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut hash_map = HashMap::new();
        $(hash_map.insert($key, $val);)*
        
        hash_map
    }}
}
struct Solution;
impl Solution {
    #[inline]
    fn clean_up_old_characters(character_set: &mut [usize], characters_to_delete: &[u8]) {
        for &character in characters_to_delete.iter() {
            character_set[character as usize] = usize::MAX;
        }
    }

    pub fn length_of_longest_substring(s: &str) -> i32 {
        // map of ASCII to index in the string
        let mut scanned_characters = [usize::MAX;256];
        
        // Treating &str as &[u8] instead of iterating over chars to save space
        // because question ensures all input is ASCII.
        // Rust chars are Unicode-like, 4 bytes long.
        let s = s.as_bytes();
        
        // Setting up window parameters.
        let mut start_ind = 0usize;
        let mut max_length = 0usize;
        for (end_ind, &curr_char) in s.iter().enumerate() {
            let curr_char = curr_char as usize;
            // A character seen before appears again..
            if scanned_characters[curr_char as usize] != usize::MAX {
                // Calculate the current window size, because we're going to be resizing it.
                max_length = max(max_length, end_ind - start_ind);
                
                let old_start_ind = start_ind;
                start_ind = scanned_characters[curr_char] + 1;
                
                // After resizing the window to exclude all characters before and including the
                // repeating one, we need to remove all the excluded ones from the map.
                Solution::clean_up_old_characters(&mut scanned_characters, &s[old_start_ind..start_ind]);
            
                // Tried the hot-path to skip if a character is repeated multiple times adjacent to itself,
                // with surprisngly no measurable improvements in speed.
                // I mean, you have to probe the hash-map uselessly,
                // Remove the old character, then add the same character again with a different index,
                // call clean_up_old_characters() just to clean ONE character every iteration,
                // BUT STILL NO MEASURABLE IMPROVEMENT AFTER ELIMINATING ALL THIS!
                // I'm just stumped as to how good the compiler optimisation is, or how good Google's 
                // hash-map implementation is.
                // That makes the code more idiomatic though, so it's all great.
            }

            scanned_characters[curr_char as usize] = end_ind;
            
        }
        // It may be the case that all characters are unique, in which case we would want to
        // calculate the max_length again here
        max(max_length, s.len() - start_ind) as i32
    }
}

fn main() {
    let tests = new_hash_map!{
        "abcabcbb" => 3,
        "bbbbb" => 1,
        "aaaaaaa" => 1,
        "aawercokaahgiiiii" => 7,
        "pwwkew" => 3,
        "" => 0,
        " " => 1,
        "abcdefghijklmnopqrstuvwxyz" => 26,
        "a" => 1,
    };

    let mut average = 0u128;
    const TRIES: usize = 32;
    for _ in 0..TRIES { 
        for (&key, &val) in tests.iter() {
            // println!("key: '{}'", key);
            let time_begin = Instant::now();
            assert_eq!(Solution::length_of_longest_substring(key), val);
            let time_spent = time_begin.elapsed();
            // println!("{}: {} ns", key, time_spent.as_nanos());
            average += time_spent.as_nanos();
        }
    }
    println!("Test passed, took: {} ns (average)", average as f32 / (tests.len() * TRIES) as f32);
    println!("Total time: {} ns", average);
}
