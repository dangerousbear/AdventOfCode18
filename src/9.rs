use std::collections::VecDeque;

fn main() {
    let n_players = 429;
    let n_marbles = 7090100;
    // let n_players = 9;
    // let n_marbles = 25;
    let mut marbles = VecDeque::with_capacity(n_marbles);

    marbles.extend([2, 1, 0]);
    let mut turn_idx = 1;
    let mut scores = vec![0; n_players];

    for count in 3..n_marbles {
        if count % 10000 == 0 {
            println!("{}", count as f32 / n_marbles as f32);
        }
        // println!("Turn idx+1, Marbles, active marble: {:?}, {:?}", turn_idx + 1, marbles);
        if count % 23 == 0 {
            marbles.rotate_right(7);
            scores[turn_idx] += count + marbles.pop_back().unwrap() as usize;
            marbles.rotate_left(1);
        } else {
            marbles.rotate_left(1);
            marbles.push_back(count);
        }
        turn_idx = (turn_idx + 1) % n_players;
    }
    println!("Highest score: {:?}", scores.iter().max().unwrap())
}
