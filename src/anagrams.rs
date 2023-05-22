use std::collections::HashMap;

fn count_char(s: &str) -> HashMap<char, i32> {
    let mut map:HashMap<char, i32>  = HashMap::new();

    for c in s.chars() {
       let entry =  map.entry(c).or_insert(0);
       *entry += 1;
    }

    map
}

fn is_anagrams(s1: &str, s2: &str) -> bool {    
    let characters_1 = count_char(s1);
    let characters_2 = count_char(s2);
    for k in characters_1.keys() {
        if !characters_2.contains_key(k) {
            return false;
        }

        if characters_1.get(k) != characters_2.get(k) {
            return false;
        }
    }

    for k in characters_2.keys() {
        if !characters_1.contains_key(k) {
            return false;
        }
    }

    true
}

fn main() {
    let a = "cat";
    let b = "taz";

    println!("{:?}", is_anagrams(a, b));
}
