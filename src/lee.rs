use std::collections::{HashMap, VecDeque};

use crate::{types::Cell, Coords};

pub type Grid = Vec<Vec<Cell>>;

pub fn lee(grid: &Grid, from: Coords, to: Coords) -> Option<Vec<Coords>> {
    let max_y = grid.len() - 1;
    let max_x = grid.first().unwrap().len() - 1;
    let max = (max_x, max_y);

    let mut queue = VecDeque::new();
    queue.push_back(from);
    let mut lengths = HashMap::new();
    lengths.insert(from, 0);

    while queue.len() > 0 {
        let cell = queue
            .pop_front()
            .expect("Failed to pop from non-empty queue");
        let cell_length = *lengths.get(&cell).expect("Failed to get cell length");
        get_candidates(cell, max).for_each(|candidate_cell| {
            let available_move =
                grid[candidate_cell.1][candidate_cell.0].used <= grid[cell.1][cell.0].size;
            let never_visited = lengths.get(&candidate_cell).is_none();
            if never_visited && available_move {
                queue.push_back(candidate_cell);
                lengths.insert(candidate_cell, cell_length + 1);
            }
        });
    }

    if lengths.get(&to).is_none() {
        return None;
    }

    let mut full_path = vec![to];
    loop {
        let last_cell = *full_path
            .last()
            .expect("Failed to get last element from full path");

        if last_cell == from {
            break;
        }

        let last_cell_length = lengths
            .get(&last_cell)
            .expect("Failed to get last cell length");
        let next_cell = get_candidates(last_cell, max)
            .find(|cell| last_cell_length - 1 == *lengths.get(cell).unwrap_or(&i32::MAX))
            .expect("Failed to trace wave back");
        full_path.push(next_cell);
    }

    Some(full_path.into_iter().rev().collect())
}

pub fn get_candidates(current: Coords, max: Coords) -> impl Iterator<Item = Coords> {
    let mut candidates = vec![];
    if current.0 > 0 {
        candidates.push((current.0 - 1, current.1));
    }
    if current.0 < max.0 {
        candidates.push((current.0 + 1, current.1));
    }
    if current.1 > 0 {
        candidates.push((current.0, current.1 - 1));
    }
    if current.1 < max.1 {
        candidates.push((current.0, current.1 + 1));
    }
    candidates.into_iter()
}

#[cfg(test)]
mod tests {
    use crate::types::Cell;

    use super::lee;

    #[test]
    pub fn test_1x1() {
        let grid = vec![vec![Cell {
            size: 12,
            used: 5,
            avail: 12 - 5,
        }]];
        let from = (0, 0);
        let to = (0, 0);

        let path = lee(&grid, from, to);

        assert_eq!(path, Some(vec![(0, 0)]));
    }

    #[test]
    pub fn test_2x2() {
        let grid = vec![
            vec![
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
            ],
            vec![
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
            ],
        ];
        let from = (0, 0);
        let to = (1, 1);

        let path = lee(&grid, from, to);

        assert_eq!(path, Some(vec![(0, 0), (0, 1), (1, 1)]));
    }

    #[test]
    pub fn test_3x3_simple() {
        let grid = vec![
            vec![
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
            ],
            vec![
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
            ],
            vec![
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 12,
                    used: 0,
                    avail: 12,
                },
            ],
        ];
        let from = (0, 0);
        let to = (2, 2);

        let path = lee(&grid, from, to);

        assert_eq!(path, Some(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]));
    }

    #[test]
    pub fn test_3x3_with_two_unavailable() {
        let grid = vec![
            vec![
                Cell {
                    size: 20,
                    used: 0,
                    avail: 12,
                },
                Cell {
                    size: 20,
                    used: 19,
                    avail: 12,
                },
                Cell {
                    size: 20,
                    used: 15,
                    avail: 12,
                },
            ],
            vec![
                Cell {
                    size: 99,
                    used: 50,
                    avail: 12,
                },
                Cell {
                    size: 99,
                    used: 50,
                    avail: 12,
                },
                Cell {
                    size: 20,
                    used: 19,
                    avail: 12,
                },
            ],
            vec![
                Cell {
                    size: 20,
                    used: 20,
                    avail: 12,
                },
                Cell {
                    size: 20,
                    used: 19,
                    avail: 12,
                },
                Cell {
                    size: 20,
                    used: 20,
                    avail: 12,
                },
            ],
        ];
        let from = (0, 0);
        let to = (0, 2);

        let path = lee(&grid, from, to);

        assert_eq!(
            path,
            Some(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2)])
        );
    }
}
