pub fn _run() {
    let input = include_str!("input")
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|coord_string| {
                    coord_string
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| (v[0], v[1]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let _max_x = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.0).max().unwrap())
        .max()
        .unwrap();
    let _min_x = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.0).min().unwrap())
        .min()
        .unwrap();

    let max_y = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.1).max().unwrap())
        .max()
        .unwrap();

    let mut grid = vec![vec![0; 1000]; max_y + 2];

    for line in input {
        let mut current = line[0];
        for segment in line {
            for x in if current.0 < segment.0 {
                current.0..=segment.0
            } else {
                segment.0..=current.0
            } {
                for y in if current.1 < segment.1 {
                    current.1..=segment.1
                } else {
                    segment.1..=current.1
                } {
                    grid[y][x] = 1;
                }
            }
            current = segment;
        }
    }

    for x in 0..1000 {
        grid[max_y + 1][x] = 1;
    }

    let mut counter = 0;
    let part1 = 'outer: loop {
        let mut coord: (usize, usize) = (500, 0);
        loop {
            if coord.1 == max_y {
                break 'outer counter;
            }
            if grid[coord.1 + 1][coord.0] == 0 {
                coord = (coord.0, coord.1 + 1);
            } else if grid[coord.1 + 1][coord.0 - 1] == 0 {
                coord = (coord.0 - 1, coord.1 + 1);
            } else if grid[coord.1 + 1][coord.0 + 1] == 0 {
                coord = (coord.0 + 1, coord.1 + 1);
            } else {
                grid[coord.1][coord.0] = 2;
                break;
            }
        }
        counter += 1;
    };

    let input = include_str!("input")
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|coord_string| {
                    coord_string
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| (v[0], v[1]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let _max_x = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.0).max().unwrap())
        .max()
        .unwrap();
    let _min_x = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.0).min().unwrap())
        .min()
        .unwrap();

    let max_y = input
        .iter()
        .map(|line| line.iter().map(|coord| coord.1).max().unwrap())
        .max()
        .unwrap();
    let mut grid = vec![vec![0; 1000]; max_y + 3];

    for line in input {
        let mut current = line[0];
        for segment in line {
            for x in if current.0 < segment.0 {
                current.0..=segment.0
            } else {
                segment.0..=current.0
            } {
                for y in if current.1 < segment.1 {
                    current.1..=segment.1
                } else {
                    segment.1..=current.1
                } {
                    grid[y][x] = 1;
                }
            }
            current = segment;
        }
    }

    for x in 0..1000 {
        grid[max_y + 2][x] = 1;
    }

    let mut counter = 0;
    let part2 = 'outer: loop {
        let mut coord: (usize, usize) = (500, 0);
        loop {
            if grid[0][500] == 2 {
                break 'outer counter;
            }
            if grid[coord.1 + 1][coord.0] == 0 {
                coord = (coord.0, coord.1 + 1);
            } else if grid[coord.1 + 1][coord.0 - 1] == 0 {
                coord = (coord.0 - 1, coord.1 + 1);
            } else if grid[coord.1 + 1][coord.0 + 1] == 0 {
                coord = (coord.0 + 1, coord.1 + 1);
            } else {
                grid[coord.1][coord.0] = 2;
                break;
            }
        }

        counter += 1;
    };

    // if counter % 10000 == 0{
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    // for row in grid.clone(){
    //     println!("{}", row[(min_x - 50) ..=(max_x + 50)].iter().map(|i| match i { 1 => '#', 2 => 'o', _ => '.'}).collect::<String>());
    // }
    // }

    println!("{}", part1);
    println!("{}", part2);
}
