//use std::env;
use std::fs;

pub fn parse_int_list(file_path: &str) -> Vec<i32> {
    // --snip--
    //    let mut file_path = "data/".to_owned();
    //    file_path.push_str(file_name);
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Failed reading file")
        .replace("\r", "");

    let strings: Vec<_> = contents.split("\n").collect();
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|x| x.parse()).collect();
    return numbers.unwrap();
    // println!("With text:\n{contents}");
}
