mod parse_utils;
use std::collections::HashMap;

fn main() {
    let mut days_to_event_minutes: HashMap<String, Vec<i32>> = HashMap::new();
    let mut days_to_guard_ids: HashMap<String, i32> = HashMap::new();

    let input = parse_utils::parse_str_list("data/4.txt");
    for line in input.iter() {
        let date = &line[line.find("[").unwrap() + 6..line.find(" ").unwrap()];
        if let Some(n) = line.find("#") {
            let id = line[n + 1..line.find(" b").unwrap()]
                .parse::<i32>()
                .unwrap();
            if line.contains("23:") {
                // Clumsy date parsing to generate next day
                let mut day = date[3..].parse::<i32>().unwrap();
                let mut month = date[..2].parse::<i32>().unwrap();
                if day == 31
                    || (day == 30 && [2, 4, 6, 9, 11].contains(&month))
                    || day == 28 && month == 2
                {
                    month += 1;
                    day = 1;
                } else {
                    day += 1;
                }
                let new_date_str = format!("{:02}-{:02}", month, day);
                println!("{}", new_date_str);
                days_to_guard_ids.insert(new_date_str, id);
            } else {
                days_to_guard_ids.insert(date.to_string(), id);
            }
        } else {
            let min = &line[line.find(":").unwrap() + 1..line.find("]").unwrap()]
                .parse::<i32>()
                .unwrap();
            days_to_event_minutes
                .entry(date.to_string())
                .and_modify(|v| v.push(*min))
                .or_insert(vec![*min]);
        }
    }
    println!(
        "{:?}, length {}",
        days_to_guard_ids,
        days_to_guard_ids.len()
    );
    println!(
        "{:?}, length {}",
        days_to_event_minutes,
        days_to_event_minutes.len()
    );
    let mut guard_id_to_sleep_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (day, mut events) in days_to_event_minutes {
        events.sort();
        let id = days_to_guard_ids
            .get(&day)
            .expect(&format!("Failed for day {}, id events {:?}", day, events));
        let n_events = events.len();
        if !guard_id_to_sleep_map.contains_key(id) {
            guard_id_to_sleep_map.insert(*id, vec![0; 60]);
        }
        guard_id_to_sleep_map.entry(*id).and_modify(|v| {
            for i in (0..n_events).step_by(2) {
                for j in events[i]..events[i + 1] {
                    v[j as usize] += 1;
                }
            }
        });
    }
    println!("{:?}", guard_id_to_sleep_map);
    let max_sleep_guard = guard_id_to_sleep_map
        .iter()
        .max_by(|a, b| a.1.iter().sum::<i32>().cmp(&b.1.iter().sum::<i32>()))
        .map(|(k, _v)| k)
        .unwrap();
    println!("Guard ID {:?}", max_sleep_guard);
    // let max_minute_slept = ;
    let max_minute = guard_id_to_sleep_map
        .get(max_sleep_guard)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .max()
        .unwrap()
        .1;
    println!("Max minute {:?}", max_minute);
    let prod = max_minute as i32 * max_sleep_guard;
    println!("Result part 1: {prod}");

    let mut max_id = 0;
    let mut max_time = 0;
    let mut max_frequency = 0;
    for (id, sleep_times) in guard_id_to_sleep_map{
        let max_local_time = sleep_times.iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .max()
        .unwrap()
        .1;
        let frequency = sleep_times[max_local_time];
        if frequency  > max_frequency {
            max_frequency = frequency ; 
            max_time = max_local_time;
            max_id = id;
        }
    }
    let answer2 = max_time as i32 * max_id;
    println!("Result part 2: {answer2}");

}
