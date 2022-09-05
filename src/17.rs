use itertools::Itertools;
use ndarray::{s, Array2};
use plotters::{prelude::*, style::full_palette::PURPLE};
use std::collections::{HashMap, HashSet};
mod parse_utils;

fn main() {
    let lines = parse_utils::parse_str_list("data/17.txt");

    let mut n_cols = 0;
    let mut n_rows = 0;
    let mut x_lines = Vec::new();
    let mut y_lines = Vec::new();
    for line in lines {
        let split = line.split(", ").collect_vec();
        assert_eq!(split.len(), 2);
        let c1 = split[0][2..].parse::<usize>().unwrap();
        let c2 = split[1][2..split[1].find(".").unwrap()]
            .parse::<usize>()
            .unwrap();
        let c3 = split[1][split[1].find(".").unwrap() + 2..]
            .parse::<usize>()
            .unwrap();
        if split[0].contains("x") {
            n_cols = std::cmp::max(n_cols, c1);
            n_rows = std::cmp::max(n_rows, c3);
            x_lines.push((c1, c2, c3));
        } else {
            assert!(split[0].contains("y"));
            n_rows = std::cmp::max(n_rows, c1);
            n_cols = std::cmp::max(n_cols, c3);
            y_lines.push((c1, c2, c3));
        }
    }
    n_rows += 1;
    n_cols += 10;
    let mut field = Array2::<u8>::zeros((n_rows, n_cols));
    for (x, y1, y2) in x_lines {
        for y in y1..=y2 {
            field[[y, x]] = 1;
        }
    }
    for (y, x1, x2) in y_lines {
        for x in x1..=x2 {
            field[[y, x]] = 1;
        }
    }

    // println!("N_rows {}", n_rows);

    // let mut water_frontline = vec![(500,1)];

    // for tick in 0..5 {
    //     for (r, c) in water_frontline {
    //         field[[r,c]] = 2;
    //     }

    // }
    let mut n_water_last = 0;
    loop {
        let mut produced_still_water = false;
        for r in 0..n_rows {
            // println!("Checkiung  {:?}", r);
            for c in 0..n_cols {
                if field[[r, c]] == 2 {
                    // println!("Found 2 at  {:?}, result below {}", (r, c), field[[r + 1, c]]);
                    // Look for trapped water.
                    if r + 1 < n_rows && [1u8, 3u8].contains(&field[[r + 1, c]]) // This can be done a lot more efficiently
                        && is_blocked(&field, r, c)
                    {
                        field[[r, c]] = 3;
                        produced_still_water = true;
                    } else {
                        field[[r, c]] = 0
                    }
                    // println!("Processed at {:?}, result {}", (r, c), field[[r, c]]);
                }
            }
        }
        // println!("Field post filter  {:?}", &field.slice(s![.., 480..]));
        pour(1, 500, &mut field);
        let n_water_new = field.iter().filter(|&&x| x == 2 || x == 3).count();
        // println!(
        //     "Field {:?}, Water volume {:?}",
        //     &field.slice(s![.., 480..]),
        //     n_water_new
        // );
        if n_water_last == n_water_new && !produced_still_water {
            println!("DONE");
            break;
        }
        n_water_last = n_water_new;
    }
    // println!("Checkiuasdasdasdng  {:?}", n_rows);
    let n_water_still = field.iter().filter(|&&x| x == 3).count();
    println!("Still water volume {:?}", n_water_still);

    plot(&field);
}

fn is_blocked(field: &Array2<u8>, r: usize, c: usize) -> bool {
    // println!("Checking block for {:?}", (r, c));
    match field.slice(s![r, c..]).iter().find_position(|&&x| x == 1) {
        Some(n) => {
            // println!("1 for {:?}", (r, c));

            n.0 < field
                .slice(s![r, c..])
                .iter()
                .find_position(|&&x| x == 0)
                .unwrap()
                .0
                && match field
                    .slice(s![r, ..=c])
                    .iter()
                    .rev()
                    .find_position(|&&x| x == 1)
                {
                    Some(m) => {
                        // println!("2 for {:?}", (r, c));
                        m.0 < field
                            .slice(s![r, ..=c])
                            .iter()
                            .rev()
                            .find_position(|&&x| x == 0)
                            .unwrap()
                            .0
                    }
                    None => false,
                }
        }
        None => false,
    }
}

fn pour(row: usize, col: usize, field: &mut Array2<u8>) {
    if row == field.dim().0 || field[[row, col]] == 1 {
        return;
    }
    // println!("Pour {row} {col} value beneath {:?}", field[[row + 1, col]]);
    field[[row, col]] = 2;
    if row + 1 == field.dim().0 {
        return;
    }
    if field[[row + 1, col]] == 0 {
        pour(row + 1, col, field);
    } else if field[[row + 1, col]] != 2 {
        if field[[row, col + 1]] == 0 {
            pour(row, col + 1, field);
        }
        if field[[row, col - 1]] == 0 {
            pour(row, col - 1, field);
        }
    }
}

fn plot(f: &Array2<u8>) {
    let file_name = "./plot.png";
    let root = BitMapBackend::new(&file_name, (3000, 8000)).into_drawing_area();

    root.fill(&WHITE).unwrap();
    let areas = root.split_by_breakpoints([400], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(400f64..660f64, 0f64..1975f64)
        .unwrap();
    // scatter_ctx
    //     .configure_mesh()
    //     .disable_x_mesh()
    //     .disable_y_mesh()
    //     .draw();
    scatter_ctx
        .draw_series(f.indexed_iter().map(|((y, x), v)| match *v {
            1 => Pixel::new((x as f64, y as f64), GREEN.filled()),
            2 => Pixel::new((x as f64, y as f64), BLUE.filled()),
            3 => Pixel::new((x as f64, y as f64), PURPLE.filled()),
            0 => Pixel::new((x as f64, y as f64), RED.filled()),
            _ => panic!(),
        }))
        .unwrap();
    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", &file_name);
}
