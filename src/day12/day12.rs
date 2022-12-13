use std::collections::{HashMap, VecDeque};

pub fn _run() {
    let mut elevations: HashMap<char, i32> = "EabcdefghijklmnopqrstuvwxyzS"
        .chars()
        .enumerate()
        .map(|t| (t.1, t.0 as i32))
        .collect();
    let grid = include_str!("input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| *elevations.get(&c).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let end = {
        let mut i = 0;
        'outer: loop {
            let mut j = 0;
            loop {
                if j == grid[0].len() {
                    break;
                };
                if grid[i][j] == 0 {
                    break 'outer (i as i32, j as i32);
                }
                j += 1;
            }
            i += 1
        }
    };

    //part1
    let start = {
        let mut i = 0;
        'outer: loop {
            let mut j = 0;
            loop {
                if j == grid[0].len() {
                    break;
                };
                if grid[i][j] == 27 {
                    break 'outer (i as i32, j as i32);
                }
                j += 1;
            }
            i += 1
        }
    };

    let mut visited = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([start]);
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let pathlenght = *visited.get(&current).unwrap();
        let neighbours = [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|d| (current.0 + d.0, current.1 + d.1))
            .filter(|n| {
                grid.get(n.0 as usize).map_or(false, |r| {
                    r.get(n.1 as usize).map_or(false, |h| {
                        *h <= grid[current.0 as usize][current.1 as usize] + 1
                    })
                })
            })
            .filter(|n| !visited.contains_key(n))
            .collect::<Vec<(i32, i32)>>();
        queue.extend(neighbours.clone());
        for neighbour in neighbours {
            visited.insert(neighbour, pathlenght + 1);
        }
    }
    let part1 = visited.get(&end).unwrap();
    println!("{}", part1);

    elevations.insert('S', 1);
    let grid = include_str!("input")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| *elevations.get(&c).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let starts = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 1)
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();
    let part2 = starts
        .iter()
        .map(|start| {
            let mut visited = HashMap::from([(*start, 0)]);
            let mut queue = VecDeque::from([*start]);
            while queue.len() > 0 {
                let current = queue.pop_front().unwrap();
                let pathlenght = *visited.get(&current).unwrap();
                let neighbours = [(-1, 0), (0, -1), (1, 0), (0, 1)]
                    .iter()
                    .map(|d| (current.0 + d.0, current.1 + d.1))
                    .filter(|n| {
                        grid.get(n.0 as usize).map_or(false, |r| {
                            r.get(n.1 as usize).map_or(false, |h| {
                                *h <= grid[current.0 as usize][current.1 as usize] + 1
                            })
                        })
                    })
                    .filter(|n| !visited.contains_key(n))
                    .collect::<Vec<(i32, i32)>>();
                queue.extend(neighbours.clone());
                for neighbour in neighbours {
                    visited.insert(neighbour, pathlenght + 1);
                }
            }
            visited.get(&end).map(|i| *i)
        })
        .filter_map(|d| d)
        .min()
        .unwrap();

    println!("{}", part2)
}
