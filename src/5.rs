mod parse_utils;
use std::collections::HashMap;

fn main() {
    let polymer = parse_utils::parse_str_list("data/5.txt")[0].to_owned();
    let mut letterCounts: HashMap<String, usize> = HashMap::new();
    for c in polymer.chars() {
        let cLower = c.to_lowercase().to_string();
        if ( letterCounts.contains_key(&cLower)) { continue; }
        println!("Processing {}", cLower);
        let cUpper = c.to_uppercase().to_string();
        let mut polymerCopy = polymer.clone().replace(&cLower, "").replace(&cUpper, "");
        letterCounts.insert(cLower, evolve(&polymerCopy));
    }
    println!("{:?}", letterCounts);
    let min_value = letterCounts.values().min().unwrap();

    println!("Result part 2: {:?}", min_value);
}

fn evolve(polymerIn: &String) -> usize {
    let mut polymer = polymerIn.clone();
    let mut length = polymer.len();
    loop {
        // println!("{:?}", polymer);
        // evolve(&mut polymer.to_string());
        let mut prev_char = ' ';
        for c in polymer.chars() {
            if c != prev_char
                && c.to_lowercase().to_string() == prev_char.to_lowercase().to_string()
            {
                let snip = String::from(prev_char) + &String::from(c);
                // println!("Removing {}", snip);
                let new_polymer = polymer.replace( &snip, "").to_owned();
                polymer = new_polymer;
                // println!("Resulting {:?}", polymer);
                break;
            }
            prev_char = c;
        }
        if polymer.len() == length {
            break;
        }
        else {
            length = polymer.len();
        }
    }
    return polymer.len();
}
