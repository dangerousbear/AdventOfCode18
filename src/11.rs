mod parse_utils;
extern crate ndarray;
use ndarray::Array2;
extern crate rayon;
use rayon::prelude::*;
// use ndarray::parallel::prelude::*;
use itertools::iproduct;
use ndarray::Axis;
use ndarray::Zip;
use std::sync::{Arc, Mutex};
// use std::collections::HashMap;

const SERIAL: i32 = 9445;

fn main() {
    let field_x_size: usize = 300;
    let field_y_size: usize = 300;

    let mut field = Array2::<i32>::zeros((field_x_size, field_y_size));

    for ((x, y), v) in field.indexed_iter_mut() {
        let rack_id = (x + 1) as i32 + 10;
        let initial_power = rack_id * (rack_id * (y + 1) as i32 + SERIAL);
        *v = initial_power / 100 % 10 - 5;
    }

    // let mut value_map : HashMap<(usize, usize), i32> = HashMap::new();
    let max_per_coord = Arc::new(Mutex::new(Array2::<(i32, usize)>::from_elem(
        (field_x_size, field_y_size),
        (0, 0),
    )));
    iproduct!((0..field_x_size), (0..field_y_size))
        .into_iter()
        .par_bridge()
        .for_each(|(xb, yb)| {
            println!("Evaluating {xb}, {yb}");
            let max_size = std::cmp::min(field_x_size - xb, field_y_size - yb);
            let mut val = field[[xb, yb]];
            let mut max_val = val;
            let mut corr_size = 1;
            for size in 1..max_size {
                val += field[[xb + size, yb + size]];
                for x in xb..xb + size {
                    val += field[[x, yb + size]];
                }
                for y in yb..yb + size {
                    val += field[[xb + size, y]];
                }

                if val > max_val {
                    max_val = val;
                    corr_size = size + 1;
                }

                max_per_coord.lock().unwrap()[[xb, yb]] = (max_val, corr_size);
            }
        });
    let lock = &max_per_coord.lock().unwrap();
    let ((max_x, max_y), (_, size)) = lock
        .indexed_iter()
        .max_by_key(|((_, _), (val, _))| val)
        .unwrap();
    println!("Result part 2: {max_x}, {max_y}, {size}");
}
