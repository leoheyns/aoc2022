use std::collections::HashSet;

pub fn _run() {
    let lines = include_str!("input").lines();
    let trees: Vec<Vec<_>> = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut visible: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..trees.len() {
        let mut max: i32 = -1;
        for j in 0..trees[0].len() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.insert((i as i32, j as i32));
            }
        }
    }

    for j in 0..trees[0].len() {
        let mut max: i32 = -1;
        for i in 0..trees.len() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.insert((i as i32, j as i32));
            }
        }
    }

    for i in 0..trees.len() {
        let mut max: i32 = -1;
        for j in (0..trees[0].len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.insert((i as i32, j as i32));
            }
        }
    }

    for j in 0..trees[0].len() {
        let mut max: i32 = -1;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                visible.insert((i as i32, j as i32));
            }
        }
    }

    println!("{}", visible.len());

    fn scenic_score(i: usize, j: usize, trees: &Vec<Vec<i32>>) -> i32 {
        let height: i32 = trees[i][j];
        let mut current = 1;

        let mut count = 0;
        for x in i + 1..trees.len() {
            count += 1;
            if trees[x][j] >= height {
                break;
            }
        }
        current *= count;
        let mut count = 0;
        for x in (0..i).rev() {
            count += 1;
            if trees[x][j] >= height {
                break;
            }
        }
        current *= count;

        let mut count = 0;
        for y in j + 1..trees[0].len() {
            count += 1;
            if trees[i][y] >= height {
                break;
            }
        }
        current *= count;

        let mut count = 0;
        for y in (0..j).rev() {
            count += 1;
            if trees[i][y] >= height {
                break;
            }
        }
        current *= count;

        return current;
    }

    let mut max = -1;
    for j in 0..trees[0].len() {
        for i in 0..trees.len() {
            let score = scenic_score(i, j, &trees);
            if score > max {
                max = score
            }
        }
    }

    println!("{}", max)
}
