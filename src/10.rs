use plotters::prelude::*;
use std::io::stdin;
use std::io::{self, Read, Write};
use std::str::{self, FromStr};
use std::{thread, time};

mod parse_utils;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    // println!("Input {:?}", parse_pair("<2,3>"));

    let lines = parse_utils::parse_str_list("data/10.txt");
    let mut points = Vec::with_capacity(lines.len());
    for line in lines {
        let pv_split = line.split("v").collect_vec();
        let pos = &pv_split[0][pv_split[0].find("<").unwrap() + 1..pv_split[0].len() - 1];
        let vel = &pv_split[1][pv_split[1].find("<").unwrap() + 1..pv_split[1].len() - 1];
        points.push(Point {
            x: pos[..pos.find(",").unwrap()].parse::<i32>().unwrap(),
            y: pos[pos.find(",").unwrap() + 1..].parse::<i32>().unwrap(),
            vx: vel[..vel.find(",").unwrap()].parse::<i32>().unwrap(),
            vy: vel[vel.find(",").unwrap() + 1..].parse::<i32>().unwrap(),
        });
        println!("{:?}", &points.last());
    }

    for p in &mut points {
        p.x += 10243 * p.vx;
        p.y += 10243 * p.vy;
    }
    // let delay = time::Duration::from_millis(10);
    let mut n = 1;
    // loop {
    // thread::sleep(delay);

    let path = format!("./plots/plot{}.png", n);
    n += 1;
    // Drawing
    let root_area = BitMapBackend::new(&path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(100..200, 145..180)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        points
            .iter()
            .map(|point| TriangleMarker::new((point.x, point.y), 5, &BLUE)),
    )
    .unwrap();

    // Advance simulation

    // for p in &mut points {
    //     p.x += p.vx;
    //     p.y += p.vy;
    // }
    // }

    // let coords: Vec<(i32, i32)> = lines
    //     .iter()
    //     .map(|line| {
    //         (
    //             line[line.find("n=<").unwrap()+3..line.find(",").unwrap()].parse::<i32>().expect("Failed x parse"),
    //             line[line.find(",").unwrap()+1..line.find(">").unwrap()].parse::<i32>().expect("Failed y parse"),
    //         )
    //     })
    //     .collect();
    // let velocities: Vec<(i32, i32)> = lines
    //     .iter()
    //     .map(|line| {
    //         (
    //             line[line.find("y=<").unwrap()+3..line[line.find(">").unwrap()+1..].find(",").unwrap()].parse::<i32>().expect("Failed x parse"),
    //             0//line[line[line.find(">").unwrap()..].find(",").unwrap()..line[line.find(">").unwrap()..].find(">").unwrap()].parse::<i32>().expect("Failed y parse"),
    //         )
    //     })
    //     .collect();
    // // let y_coords : Vec::<i32> = lines.iter().map(|line| ).collect();

    // println!("Result part 1: {:?}, {:?}", coords, velocities);

    // let max_x_coord = coords.iter().max_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0 + 1;
    // let max_y_coord = coords.iter().max_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1 + 1;
    // // println!("coords: {:?}", max_y_coord);
    // let mut field = Array2::<i32>::zeros((max_x_coord as usize, max_y_coord as usize));
    // for ((x, y), value) in field.indexed_iter_mut() {
    //     let mut min_dist = std::i32::MAX;
    //     for (i, (xc, yc)) in coords.iter().enumerate() {
    //         let dist = (x as i32 - xc).abs() + (y as i32 - yc).abs();
    //         if dist < min_dist {
    //             *value = i as i32;
    //             min_dist = dist;
    //         }
    //         // println!("Value at {}, {}: {}", x,y,value);
    //     }
    // }
}
