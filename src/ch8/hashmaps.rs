/// # Ch8.3 - Storing Keys with Associated Values in Hash Maps
/// * HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, SipHash by default
/// * the hashing function determines how it places these keys and values into memory
/// * both keys and values are homogeneous: same types
/// * growable but keys must be unique
/// * peers in other language - hash, map, object, hash table, dictionary, or associative array
/// 
use std::collections::HashMap;
#[derive(Debug)]
pub struct HashMaps{}

impl HashMaps{
    pub fn print(&self) {
        println!("\n======The note on HashMap======");
    // Create a New Hash Map
        // - together with insert, compiler can infer types for K and V; otherwise need to annotate
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("\nscores is now: {:?}", scores);

    // Accessing Values in a Hash Map
        // - individual key
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("\nScore for team {team_name} is {score}");

        let team_name = String::from("Black");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("Score for team {team_name} is {score}");
        // - loop over entire hashmap
            // - order is arbitrary
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    
    // HashMap and Ownership
        // - for types that implement Copy trait, like i32, values are copied into hash map
        // - for others like String, the values will be moved and hash map will be the onwer
        let new_team_name = String::from("Red");
        let new_team_score = 99;
        scores.insert(new_team_name, new_team_score);
            // - e.g., new_team_name are not usable from here on
        // println!("{new_team_name}");
            // - e.g., new_team_score are usable from here on
        println!("\nnew_team_score is still usable: {new_team_score}");

    // Updating a Hash Map
        // - overwriting a value
        println!("\n{:?}", scores);
        scores.insert(String::from("Blue"), 11);
        println!("{:?}", scores);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);

        // - adding a key:value only if the key is not present
            // - Yellow is present so do nothing
        scores.entry(String::from("Yellow")).or_insert(49);
            // - Green is not prsent so insert an entry for it
        scores.entry(String::from("Green")).or_insert(39);
        println!("\nAfter .entry(), scores is: {:?}", scores);

        // - updating a value based on old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            // - or_insert returns a mutable ref to the value for specified key
                // - i.e., in this case, 0 or existing count
            let count = map.entry(word).or_insert(0);
                // - update the pointee
            *count += 1;
        }
        println!("\n{:?}", map);

    }
}
