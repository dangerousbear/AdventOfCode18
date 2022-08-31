use itertools::Itertools;
use std::collections::{HashMap, HashSet};
mod parse_utils;

fn main() {
    let lines = parse_utils::parse_str_list("data/16.txt");
    let mut samples = Vec::<(Vec<usize>, (usize, usize, usize, usize), Vec<usize>)>::new();
    let mut test_program: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut reading_samples = true;
    for (i, line) in lines.iter().enumerate() {
        if reading_samples {
            if line.contains("Before") {
                samples.push((
                    line[line.find("[").unwrap() + 1..line.len() - 1]
                        .split(", ")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect_vec(),
                    (0, 0, 0, 0),
                    Vec::new(),
                ));
            } else if line.contains("After") {
                samples.last_mut().unwrap().2 = line[line.find("[").unwrap() + 1..line.len() - 1]
                    .split(", ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect_vec();
            } else if line.len() > 4 {
                samples.last_mut().unwrap().1 = line
                    .split(" ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
            } else if line.contains("X") {
                reading_samples = false;
            }
        } else if line.len() > 4 {
            test_program.push(
                line.split(" ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        }
    }
    let fun_names = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];
    let mut num2names = HashMap::new();
    for i in 0..16 {
        num2names.insert(i as usize, HashSet::new());
    }

    for (registers_before, instruction, registers_after) in samples {
        let (fun_num, a, b, c) = instruction;
        for (i, name) in fun_names.iter().enumerate() {
            let mut registers = registers_before.clone();
            apply_fun(name, a, b, c, &mut registers);
            if registers == registers_after {
                // println!("Match for {}", name);
                num2names
                    .get_mut(&fun_num)
                    .unwrap()
                    .insert(String::from(*name));
            }
        }
    }
    let mut num2names_final = HashMap::new();
    while num2names_final.len() < 16 {
        for (k, v) in &num2names {
            if v.len() == 1 {
                let clone = v.clone();
                let first = clone.iter().collect_vec()[0];
                num2names_final.insert(*k, String::from(first));
            }
        }
        for (k, v) in &mut num2names {
            v.retain(|x| !num2names_final.iter().any(|(k, v)| v.contains(x)));
        }
    }
    let mut registers = vec![0usize; 4];
    for (fun_num, a, b, c) in test_program {
        apply_fun(
            num2names_final.get(&fun_num).unwrap(),
            a,
            b,
            c,
            &mut registers,
        );
    }

    println!("Result {}", registers[0]);
}

fn apply_fun(name: &str, a: usize, b: usize, c: usize, r: &mut Vec<usize>) {
    match name {
        "addr" => r[c] = r[a] + r[b],
        "addi" => r[c] = r[a] + b,
        "mulr" => r[c] = r[a] * r[b],
        "muli" => r[c] = r[a] * b,
        "banr" => r[c] = r[a] & r[b],
        "bani" => r[c] = r[a] & b,
        "borr" => r[c] = r[a] | r[b],
        "bori" => r[c] = r[a] | b,
        "setr" => r[c] = r[a],
        "seti" => r[c] = a,
        "gtir" => r[c] = if a > r[b] { 1 } else { 0 },
        "gtri" => r[c] = if r[a] > b { 1 } else { 0 },
        "gtrr" => r[c] = if r[a] > r[b] { 1 } else { 0 },
        "eqir" => r[c] = if a == r[b] { 1 } else { 0 },
        "eqri" => r[c] = if r[a] == b { 1 } else { 0 },
        "eqrr" => r[c] = if r[a] == r[b] { 1 } else { 0 },
        _ => panic!("Invalid name"),
    }
}
