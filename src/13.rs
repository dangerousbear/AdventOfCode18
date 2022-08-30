use itertools::Itertools;
use ndarray::Array2;
mod parse_utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum NextIntersectionDir {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Cart {
    row: usize,
    col: usize,
    dir: Dir,
    turn_dir: NextIntersectionDir,
    occupied_char: char,
}

impl Cart {
    fn representation(&self) -> char {
        match self.dir {
            Dir::East => '>',
            Dir::North => '^',
            Dir::West => '<',
            Dir::South => 'v',
        }
    }
    fn turn_right(&mut self) {
        // println!("Called right turn for row {:?}", self.row);
        match self.dir {
            Dir::East => self.dir = Dir::South,
            Dir::North => self.dir = Dir::East,
            Dir::West => self.dir = Dir::North,
            Dir::South => self.dir = Dir::West,
        }
    }

    fn turn_left(&mut self) {
        // println!("Called left turn for row {:?}", self.row);
        match self.dir {
            Dir::East => self.dir = Dir::North,
            Dir::North => self.dir = Dir::West,
            Dir::West => self.dir = Dir::South,
            Dir::South => self.dir = Dir::East,
        }
    }

    fn turn_at_intersection(&mut self) {
        match self.turn_dir {
            NextIntersectionDir::Left => {
                self.turn_left();
                self.turn_dir = NextIntersectionDir::Straight;
            }
            NextIntersectionDir::Straight => {
                self.turn_dir = NextIntersectionDir::Right;
            }
            NextIntersectionDir::Right => {
                self.turn_right();
                self.turn_dir = NextIntersectionDir::Left;
            }
        }
    }
}

fn main() {
    let lines = parse_utils::parse_str_list("data/13.txt");
    let mut carts = Vec::new();

    let mut field = Array2::<char>::from_elem((lines.len(), lines[0].len()), ' ');

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            field[[row, col]] = c;
            match c {
                '<' => carts.push(Cart {
                    row: row,
                    col: col,
                    dir: Dir::West,
                    turn_dir: NextIntersectionDir::Left,
                    occupied_char: '-',
                }),
                '^' => carts.push(Cart {
                    row: row,
                    col: col,
                    dir: Dir::North,
                    turn_dir: NextIntersectionDir::Left,
                    occupied_char: '|',
                }),
                '>' => carts.push(Cart {
                    row: row,
                    col: col,
                    dir: Dir::East,
                    turn_dir: NextIntersectionDir::Left,
                    occupied_char: '-',
                }),
                'v' => carts.push(Cart {
                    row: row,
                    col: col,
                    dir: Dir::South,
                    turn_dir: NextIntersectionDir::Left,
                    occupied_char: '|',
                }),
                _ => (),
            }
        }
    }
    for i in 0..100000 {
        if carts.len() == 1 {
            println!("Final cart position {:?}, {:?}", carts[0].col, carts[0].row);
            break;
        }
        let mut crash_positions = Vec::new();
        for c in &mut carts {
            if crash_positions.contains(&(c.row, c.col)) {
                continue; // Already crashed
            }
            let prev_cell = &mut field[[c.row, c.col]];
            match c.dir {
                Dir::East => c.col += 1,
                Dir::North => c.row -= 1,
                Dir::West => c.col -= 1,
                Dir::South => c.row += 1,
            }
            *prev_cell = c.occupied_char;
            let next_cell = &mut field[[c.row, c.col]];
            let mut crashed = false;
            match next_cell {
                '-' | '|' => (),
                '+' => c.turn_at_intersection(),
                '/' => match c.dir {
                    Dir::West | Dir::East => c.turn_left(),
                    Dir::North | Dir::South => c.turn_right(),
                },
                '\\' => match c.dir {
                    Dir::West | Dir::East => c.turn_right(),
                    Dir::North | Dir::South => c.turn_left(),
                },
                '>' | '<' | '^' | 'v' => {
                    println!("Crash at {:?}, {:?}", c.col, c.row);
                    crashed = true;
                }
                _ => panic!("Invalid char"),
            }

            if crashed {
                c.occupied_char = ' ';
                crash_positions.push((c.row, c.col));
            } else {
                c.occupied_char = *next_cell;
                *next_cell = c.representation();
            }
        }
        for (row, col) in crash_positions {
            let crashed_carts = &carts
                .iter()
                .filter(|c| c.row == row && c.col == col)
                .collect_vec();
            assert_eq!(crashed_carts.len(), 2);
            let first_cart = crashed_carts
                .iter()
                .filter(|&&c| !['>', '<', '^', 'v', ' '].contains(&c.occupied_char))
                .collect_vec();
            assert_eq!(first_cart.len(), 1);
            field[[row, col]] = first_cart.first().unwrap().occupied_char;
            carts.retain(|c| !(c.row == row && c.col == col));
        }
        carts.sort();
    }
}
