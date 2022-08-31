use itertools::Itertools;
use ndarray::Array2;
mod parse_utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Dir {
    North,
    West,
    East,
    South,
    None,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Unit {
    row: usize,
    col: usize,
    hp: i32,
    ap: i32,
    race: Race,
}

impl Unit {
    fn representation(&self) -> char {
        match self.race {
            Race::Elf => 'E',
            Race::Goblin => 'G',
        }
    }

    fn is_alive(&self) -> bool {
        self.hp >= 0
    }

    fn dist(&self, u: &Unit) -> i32 {
        (self.row as i32 - u.row as i32).abs() + (self.col as i32 - u.col as i32).abs()
    }

    fn find_shortest_paths(&self, field: &Array2<char>) -> Array2<(usize, Dir)> {
        let mut min_dist_map =
            Array2::<(usize, Dir)>::from_elem(field.dim(), (usize::MAX, Dir::None));
        Self::search_shortest_path_inner(
            field,
            &mut min_dist_map,
            (self.row + 1, self.col),
            1,
            Dir::South,
        );
        Self::search_shortest_path_inner(
            field,
            &mut min_dist_map,
            (self.row - 1, self.col),
            1,
            Dir::North,
        );
        Self::search_shortest_path_inner(
            field,
            &mut min_dist_map,
            (self.row, self.col + 1),
            1,
            Dir::East,
        );
        Self::search_shortest_path_inner(
            field,
            &mut min_dist_map,
            (self.row, self.col - 1),
            1,
            Dir::West,
        );
        min_dist_map
    }

    fn search_shortest_path_inner(
        field: &Array2<char>,
        min_dist_map: &mut Array2<(usize, Dir)>,
        (row, col): (usize, usize),
        d: usize,
        dir: Dir,
    ) {
        if field[[row, col]] != '.' || min_dist_map[[row, col]] <= (d, dir) {
            return;
        }
        // println!("Searching {:?}", (row, col));
        min_dist_map[[row, col]] = (d, dir);
        Self::search_shortest_path_inner(field, &mut *min_dist_map, (row + 1, col), d + 1, dir);
        Self::search_shortest_path_inner(field, &mut *min_dist_map, (row - 1, col), d + 1, dir);
        Self::search_shortest_path_inner(field, &mut *min_dist_map, (row, col + 1), d + 1, dir);
        Self::search_shortest_path_inner(field, &mut *min_dist_map, (row, col - 1), d + 1, dir);
    }
}

const MAX_HP: i32 = 200;
const ATTACK_POWER: i32 = 3;
fn main() {
    let mut i = 4;
    while !run_simul(i) {
        println!("Running with AP {i}");
        i += 1
    }
    println!("Won at AP {i}");
}
fn run_simul(elf_ap: i32) -> bool {
    let lines = parse_utils::parse_str_list("data/15.txt");
    let mut units = Vec::new();

    let mut field = Array2::<char>::from_elem((lines.len(), lines[0].len()), ' ');
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            field[[row, col]] = c;
            if c == 'E' || c == 'G' {
                units.push(Unit {
                    row: row,
                    col: col,
                    hp: MAX_HP,
                    ap: if c == 'G' { ATTACK_POWER } else { elf_ap },
                    race: if c == 'G' { Race::Goblin } else { Race::Elf },
                });
            }
        }
    }

    // println!("Units {:?}", enemies.len());
    units.sort();
    println!("Units {:?}", &units);
    for round in 0..500 {
        for i in 0..units.len() {
            let u = &units[i];
            if !u.is_alive() {
                continue;
            }

            let enemy_race = if u.race == Race::Elf {
                Race::Goblin
            } else {
                Race::Elf
            };
            let enemies = units
                .iter()
                .filter(|e| e.is_alive() && e.race == enemy_race)
                .collect_vec();
            if enemies.is_empty() {
                let hp_sum = units
                    .iter()
                    .fold(0, |s, x| if x.is_alive() && x.race == x.race { s + x.hp } else { s });
                let prod = hp_sum * round;
                println!(
                    "Battle won by {:?} ! Number of rounds {:?}, hp sum {:?}, product {:?}, units {:?}, field {:?}",
                    u.race, round, hp_sum, prod, units, &field
                );
                return true;
            }
            if !enemies.iter().any(|e| u.dist(&e) == 1) {
                // No enemy in range, time to move
                let mut possible_attack_coords = enemies
                    .iter()
                    .flat_map(|e| {
                        [
                            (e.row + 1, e.col),
                            (e.row - 1, e.col),
                            (e.row, e.col - 1),
                            (e.row, e.col + 1),
                        ]
                    })
                    .filter(|(row, col)| field[[*row, *col]] == '.')
                    .collect_vec();
                if possible_attack_coords.is_empty() {
                    continue; // No possible attack square, do nothing.
                }
                let dist_map = u.find_shortest_paths(&field);
                possible_attack_coords.sort();
                let (tx, ty) = *possible_attack_coords
                    .iter()
                    .min_by(|(r1, c1), (r2, c2)| {
                        dist_map[[*r1, *c1]].0.cmp(&dist_map[[*r2, *c2]].0)
                        // .then(r1.cmp(r1))
                        // .then(c1.cmp(c2)))
                    })
                    .unwrap();

                // Perform the move
                field[[u.row, u.col]] = '.';
                let mut u = &mut units[i];
                match dist_map[[tx, ty]].1 {
                    Dir::North => u.row -= 1,
                    Dir::South => u.row += 1,
                    Dir::East => u.col += 1,
                    Dir::West => u.col -= 1,
                    Dir::None => (),
                }
                field[[u.row, u.col]] = u.representation();
            }

            // Try to attack
            let u = &units[i];
            let ap = u.ap;
            let (mut a_col, mut a_row) = (0, 0);
            if let Some(attacked_unit) = units
                .iter()
                .filter(|e| e.is_alive() && e.race == enemy_race && u.dist(&e) == 1)
                .min_by_key(|e| e.hp)
            {
                (a_row, a_col) = (attacked_unit.row, attacked_unit.col);
                if attacked_unit.hp <= ap {
                    if attacked_unit.race == Race::Elf {
                        println!("Elf death!");
                        return false;
                    }
                    println!("Death! at {a_row}, {a_col}");
                    field[[a_row, a_col]] = '.';
                }
            }
            if a_row != 0 && a_col != 0 {
                for au in &mut units {
                    if au.row == a_row && au.col == a_col {
                        au.hp -= ap;
                        break;
                    }
                }
            }
        }

        units.retain(|u| u.is_alive());
        println!(
            "After round {:?}//////////////////////////////////\n Field {:?}",
            round + 1,
            &field
        );
        units.sort();
        println!("Units {:?}", &units);
    }
    panic!("Did not end!");
}
