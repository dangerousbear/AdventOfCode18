mod parse_utils;
use std::collections::HashSet;

fn main() {
    let numbers = parse_utils::parse_int_list("data/1.txt", "\n");
    let answer : i32 = numbers.iter().sum();
    println!("Result part 1: {answer}");
    let mut frequencies : HashSet<i32> = HashSet::new();
    let mut i = 0;
    let mut f = 0;
    loop {
        if frequencies.contains(&f) {
            break;
        }
        frequencies.insert(f);
        f += numbers[i];
        i = if i + 1 < numbers.len() { i + 1 } else { 0 };
    }
    println!("Result part 2: {f}");
}
