use std::collections::{HashMap, VecDeque};

use lee::get_candidates;
use types::Cell;
pub type Coords = (usize, usize);

use crate::parser::parse_input;

mod lee;
mod parser;
mod types;

// fn is_viable_pair(cellA: &Cell, cellB: &Cell) -> bool {
//     let cell_a_is_not_empty = cellA.used > 0;
//     let data_a_fits_node_b = cellA.used <= cellB.avail;

//     cell_a_is_not_empty && data_a_fits_node_b
// }

fn transform_to_grid(raw: Vec<(u8, u8, types::Cell)>) -> (Vec<Vec<Cell>>, Coords) {
    let max = raw
        .clone()
        .into_iter()
        .fold((0, 0), |(max_x, max_y), (x, y, _)| {
            ((x as usize).max(max_x), (y as usize).max(max_y))
        });

    let mut grid = vec![
        vec![
            Cell {
                size: 0,
                used: 0,
                avail: 0
            };
            max.0 + 1
        ];
        max.1 + 1
    ];
    let mut empty = (0, 0);

    raw.into_iter().for_each(|(x, y, raw_cell)| {
        let x = x as usize;
        let y = y as usize;

        if raw_cell.used == 0 {
            empty = (x, y);
        };
        grid[y][x] = raw_cell;
    });

    (grid, empty)
}

fn dp(grid: &Vec<Vec<Cell>>, empty: Coords, start: Coords, target: Coords) -> usize {
    let max_y = grid.len() - 1;
    let max_x = grid.first().unwrap().len() - 1;
    let max = (max_x, max_y);

    let mut dp = HashMap::new();
    dp.insert(start, 0);
    let mut queue = VecDeque::new();
    queue.push_front((start, empty));

    while queue.len() > 0 {
        // println!("dp table size: {}", dp.len());
        let (cell, empty) = queue.pop_back().expect("Unexpected empty queue");
        let min_path = *dp
            .get(&cell)
            .expect("Failed to get already visited cell path length");

        get_candidates(cell, max)
            .filter(|(x, y)| grid[*y][*x].used <= grid[cell.1][cell.0].size)
            .for_each(|candidate_cell| {
                let old_min = dp.get(&candidate_cell).unwrap_or(&usize::MAX);
                if &(min_path + 1) < old_min {
                    let mut grid = grid.clone();
                    grid[cell.1][cell.0] = Cell {
                        size: 0,
                        used: 0,
                        avail: 0,
                    };
                    let path = lee::lee(&grid, empty, candidate_cell);

                    if let Some(path) = path {
                        match dp.get(&candidate_cell) {
                            None => {
                                dp.insert(candidate_cell, min_path + path.len());
                                queue.push_front((candidate_cell, cell));
                            }
                            Some(dp_min) => {
                                if dp_min < &(min_path + path.len()) {
                                    dp.insert(candidate_cell, min_path + path.len());
                                    queue.push_front((candidate_cell, cell));
                                }
                            }
                        };
                    }
                }
            });
    }

    print_dp(&dp, max);
    *dp.get(&target).unwrap()
}

fn print_dp(grid: &HashMap<Coords, usize>, max: Coords) {
    for i in 0..=max.0 {
        for j in 0..=max.1 {
            print!("{:4}", grid.get(&(j, i)).unwrap_or(&0));
        }
        println!("");
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    println!(">>>>>>>> USED");
    for row in grid.into_iter() {
        for cell in row.into_iter() {
            print!("{:4}", cell.used);
        }
        println!("");
    }
    println!(">>>>>>>> SIZE");
    for row in grid.into_iter() {
        for cell in row.into_iter() {
            print!("{:4}", cell.size);
        }
        println!("");
    }
    println!("----------");
}

fn main() {
    let (_, raw_grid) = parse_input(include_str!("input.txt")).expect("Failed to parse input");

    let (grid, empty) = transform_to_grid(raw_grid);
    print_grid(&grid);

    let a = dp(&grid, empty, (33, 0), (0, 0));
    println!("{:?}", a);
    // let mut viable_pairs_counter = 0;
    // for (x1, y1) in grid.keys() {
    //     for (x2, y2) in grid.keys() {
    //         if (x1, y1) != (x2, y2) {
    //             let cell_a = grid.get(&(*x1, *y1)).expect("Failed to get cellA");
    //             let cell_b = grid.get(&(*x2, *y2)).expect("Failed to get cellB");
    //             if is_viable_pair(cell_a, cell_b) {
    //                 viable_pairs_counter += 1;
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", grid);
    // println!("Viable pairs: {}", viable_pairs_counter);
}
