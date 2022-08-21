//use std::env;
use std::fs;

pub fn parse_int_list(file_path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file")
        .replace("\r", "");

    let strings: Vec<_> = contents.split("\n").collect();
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|x| x.parse()).collect();
    return numbers.unwrap();
    // println!("With text:\n{contents}");
}

pub fn parse_str_list(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file")
        .replace("\r", "");
    return contents.split("\n").map(|s| s.to_string()).collect();
    // println!("With text:\n{contents}");
}