mod parse_utils;
use std::collections::HashMap;
fn main() {
    let ids = parse_utils::parse_str_list("data/2.txt");
    let mut sum2 = 0;
    let mut sum3 = 0;
    for (i, id) in ids.iter().enumerate() {
        let mut letterCounts: HashMap<char, i32> = HashMap::new();
        for c in id.chars() {
            letterCounts
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        if letterCounts.values().any(|&v| v == 2) {
            sum2 += 1;
        };
        if letterCounts.values().any(|&v| v == 3) {
            sum3 += 1;
        };

        // Brute force check all pairs for part 2

        for id2 in &ids[i..] {
            let mut num_diffs = 0;
            for (c1, c2) in id.chars().zip(id2.chars()) {
                if c1 != c2 {
                    num_diffs += 1;
                }
            }
            if num_diffs == 1 {
                println!("Result part 2: Words {id}, {id2}")
            }
        }

        // println!("{:?}", letterCounts);
    }
    let answer = sum2 * sum3;
    println!("Result part 1: {answer}");
}
