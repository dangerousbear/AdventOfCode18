mod parse_utils;
extern crate ndarray;
use ndarray::{Array2, Dim};

fn main() {
    let input = parse_utils::parse_str_list("data/3.txt");  
    let mut x_coords:Vec<i32> = Vec::new();
    let mut y_coords:Vec<i32> = Vec::new();
    let mut x_sizes:Vec<i32> = Vec::new();
    let mut y_sizes:Vec<i32> = Vec::new();
    let mut field_x_size = 0;
    let mut field_y_size = 0;
    let num_claims = input.len();
    for line in input.iter(){
        x_coords.push(line[line.find("@").unwrap()+2..line.find(",").unwrap()].parse::<i32>().unwrap());
        y_coords.push(line[line.find(",").unwrap()+1..line.find(":").unwrap()].parse::<i32>().unwrap());
        x_sizes.push(line[line.find(":").unwrap()+2..line.find("x").unwrap()].parse::<i32>().unwrap());
        y_sizes.push(line[line.find("x").unwrap()+1..].parse::<i32>().unwrap());
        field_x_size = field_x_size.max(x_coords.last().copied().unwrap() + x_sizes.last().copied().unwrap());
        field_y_size = field_y_size.max(y_coords.last().copied().unwrap() + y_sizes.last().copied().unwrap());
    }

    let mut field = Array2::<i32>::zeros((field_x_size as usize, field_y_size as usize));

    for i in 0..num_claims{
        for x in x_coords[i]..x_coords[i]+x_sizes[i] {
            for y in y_coords[i]..y_coords[i]+y_sizes[i] {
                field[[x as usize,y as usize]] += 1;
            }
        }
    }
    println!("{:?}", field);
    let answer = field.iter().filter(|v| **v>1).count();
    println!("Result part 1: {answer}");
    for i in 0..num_claims{
        let mut no_collision = true;
        for x in x_coords[i]..x_coords[i]+x_sizes[i] {
            for y in y_coords[i]..y_coords[i]+y_sizes[i] {
                no_collision &= field[[x as usize,y as usize]] == 1;
            }
        }
        if no_collision {
            println!("Result part 2, index of free claim: {i}");
        }
    }
}
