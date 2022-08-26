mod parse_utils;
use multimap::MultiMap;

fn main() {
    let lines = parse_utils::parse_str_list("data/7.txt");
    let rules: Vec<(char, char)> = lines
        .iter()
        .map(|line| {
            (
                line[line.find("Step").unwrap() + 5..line.find(" m").unwrap()]
                    .parse::<char>()
                    .unwrap(),
                line[line.find("step").unwrap() + 5..line.find(" c").unwrap()]
                    .parse::<char>()
                    .unwrap(),
            )
        })
        .collect();

    let forward_map: MultiMap<_, _> = rules.iter().cloned().collect();
    let backward_map: MultiMap<_, _> = rules.iter().map(|(a, b)| (b, a)).collect();

    let mut options = Vec::new();
    for c in forward_map.keys() {
        if !backward_map.contains_key(c) {
            options.push(*c);
        }
    }

    let n_workers = 5;
    let mut time_to_ready = vec![0; n_workers];
    let mut char_being_worked_on = vec![' '; n_workers];
    let mut t = 0;

    // Can probably be significantly improved with a better data structure (ordered set)
    println!("Initial options {:?}", options);
    options.sort();
    let mut path: String = String::from("");
    let char_shift = 'A' as i32 - 1;
    loop {
        for i in 0..n_workers {
            if time_to_ready[i] == 1 {
                path.push(char_being_worked_on[i]);
                char_being_worked_on[i] = ' ';
            }
            time_to_ready[i] -= 1;
        }
        for pc in path.chars() {
            if forward_map.contains_key(&pc) {
                options.extend(forward_map.get_vec(&pc).unwrap().iter().filter(|p| {
                    !path.contains(**p)
                        && !char_being_worked_on.contains(*p)
                        && backward_map
                            .get_vec(*p)
                            .unwrap()
                            .iter()
                            .all(|x| path.contains(**x))
                }));
            }
        }
        if options.is_empty() && char_being_worked_on.iter().all(|c| *c == ' ') {
            break;
        }
        options.sort();
        options.dedup();
        for i in 0..n_workers {
            if time_to_ready[i] <= 0 && !options.is_empty() {
                time_to_ready[i] = 60 + options[0] as i32 - char_shift;
                char_being_worked_on[i] = options[0];
                options.remove(0);
            }
        }
        t += 1;
    }
    println!("Result part 2: {:?}", t);
}
