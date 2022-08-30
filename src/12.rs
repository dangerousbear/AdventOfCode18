use itertools::Itertools;
mod parse_utils;

fn main() {
    let lines = parse_utils::parse_str_list("data/12.txt");
    let mut state = Vec::new();
    let mut rules = Vec::new();
    for (i, line) in lines
        .iter()
        .map(|line| line.replace("#", &"1").replace(".", &"0"))
        .enumerate()
    {
        if i == 0 {
            state = line[line.find(":").unwrap() + 2..]
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
        } else {
            let side_split = line.split(" => ").collect_vec();
            rules.push((
                side_split[0]
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec(),
                side_split[1].parse::<u8>().unwrap(),
            ));
        }
    }

    let mut offset: i64 = 0;
    let mut n = 0;
    let five_hundred_billion: i64 = 50000000000;

    for gen in 0..300 {
        println!("{:?}", state.len());
        for _ in 0..4 {
            state.insert(0, 0);
        }
        state.extend([0, 0, 0, 0]);

        let mut new_state = vec![0; state.len() + 4];
        for i in 2..state.len() - 2 {
            for (lhs, res) in &rules {
                if &state[i - 2..=i + 2] == lhs {
                    new_state[i] = *res;
                }
            }
        }
        offset -= 4;
        while new_state[0] == 0 {
            offset += 1;
            new_state.remove(0);
        }
        while *new_state.last().unwrap() == 0 as u8 {
            new_state.pop();
        }
        n += 1;
        if new_state == state[4..state.len() - 4] {
            state = state[4..state.len() - 4].to_vec();
            print!("Done at gen {gen}");
            offset += five_hundred_billion - n;
            break; // From here on, we will just have translation to the right
        }
        state = new_state;
    }
    println!("{:?}", state);

    let result = state.iter().enumerate().fold(
        0,
        |s, (i, x)| if *x == 1 { s + i as i64 + offset } else { s },
    );
    println!("Result part 1: {result}, {offset}");
}
