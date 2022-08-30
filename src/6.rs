mod parse_utils;
use ndarray::{Array2, Dim};

fn main() {
    let lines = parse_utils::parse_str_list("data/6.txt");
    let coords: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            (
                line[0..line.find(",").unwrap()]
                    .parse::<i32>()
                    .expect("Failed x parse"),
                line[line.find(",").unwrap() + 2..]
                    .parse::<i32>()
                    .expect("Failed y parse"),
            )
        })
        .collect();
    // let y_coords : Vec::<i32> = lines.iter().map(|line| ).collect();

    // println!("Result part 1: {:?}", );

    let max_x_coord = coords.iter().max_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0 + 1;
    let max_y_coord = coords.iter().max_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1 + 1;
    // println!("coords: {:?}", max_y_coord);
    let mut field = Array2::<i32>::zeros((max_x_coord as usize, max_y_coord as usize));
    for ((x, y), value) in field.indexed_iter_mut() {
        let mut min_dist = std::i32::MAX;
        for (i, (xc, yc)) in coords.iter().enumerate() {
            let dist = (x as i32 - xc).abs() + (y as i32 - yc).abs();
            if dist < min_dist {
                *value = i as i32;
                min_dist = dist;
            }
            // println!("Value at {}, {}: {}", x,y,value);
        }
    }

    let mut touches_edge = vec![false; coords.len()];
    let mut num_elements = vec![0; coords.len()];

    for ((y, x), value) in field.indexed_iter() {
        if x == 0 || y == 0 || x == max_x_coord as usize || y == max_y_coord as usize {
            touches_edge[*value as usize] = true;
        }
        num_elements[*value as usize] += 1;
    }
    let max_finite_area = num_elements
        .iter()
        .zip(touches_edge.iter())
        .map(|(n, touches)| if *touches { 0 } else { *n })
        .max()
        .unwrap();

    println!("Touches edge {:?}", touches_edge);
    println!("Num elements{:?}", num_elements);
    println!("Result part 1: {:?}", max_finite_area);

    let mut field2 = Array2::<i32>::zeros((max_x_coord as usize, max_y_coord as usize));
    for ((x, y), value) in field2.indexed_iter_mut() {
        for (xc, yc) in &coords {
            *value += (x as i32 - xc).abs() + (y as i32 - yc).abs();
        }
    }
    println!("field2: {:?}", field2);
    let safe_area = field2.iter().filter(|v| **v < 10000).count();
    println!("Result part 2: {:?}", safe_area);
}
