const MATCH: [u8; 6] = [6, 3, 3, 6, 0, 1];
fn main() {
    let N = 100000000;
    let mut scores = vec![3 as u8, 7 as u8];
    scores.reserve(N + 10);
    let mut first_idx = 0;
    let mut second_idx = 1;
    for i in 0..(N + 10) {
        let sum = scores[first_idx] + scores[second_idx];
        for c in sum.to_string().chars() {
            scores.push(c.to_digit(10).unwrap() as u8);
        }
        first_idx = (first_idx + scores[first_idx] as usize + 1) % scores.len();
        second_idx = (second_idx + scores[second_idx] as usize + 1) % scores.len();
        if i > 5
            && (scores[scores.len() - MATCH.len()..] == MATCH
                || scores[scores.len() - MATCH.len() - 1..scores.len() - 1] == MATCH)
        {
            let n = if scores[scores.len() - MATCH.len()..] == MATCH {
                scores.len() - MATCH.len()
            } else {
                scores.len() - 1 - MATCH.len()
            };
            println!("Found after {:?} ", n);
            break;
        }
    }

    // let N = 633601;
    // let mut scores = vec![3 as u8, 7 as u8];
    // scores.reserve(N+10);
    // let mut first_idx = 0;
    // let mut second_idx = 1;
    // for _ in 0..(N+10) {
    //     // println!("Scores {:?}", scores);
    //     let sum = scores[first_idx] + scores[second_idx];
    //     for c in sum.to_string().chars() {
    //         scores.push(c.to_digit(10).unwrap() as u8);
    //     }
    //     first_idx = (first_idx + scores[first_idx] as usize + 1 ) % scores.len();
    //     second_idx = (second_idx + scores[second_idx] as usize + 1 ) % scores.len();
    // }
    // println!("Result part 1 {:?}", &scores[N..N+10]);
}
