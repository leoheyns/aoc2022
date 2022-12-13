pub fn _run() {
    let lines = include_str!("input").lines();
    let mut states = lines
        .scan((1 as i32, 1 as i32), |s, l| {
            match l {
                "noop" => s.0 += 1,
                addx => {
                    s.0 += 2;
                    s.1 += addx.split(" ").collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap()
                }
            };
            Some(*s)
        })
        .collect::<Vec<(i32, i32)>>();
    states.insert(0, (1, 1));

    //should probably put the states into a hashmap from cycle to x
    let part1: i32 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| {
            states
                .clone()
                .into_iter()
                .filter(|s| s.0 == i || s.0 == i - 1)
                .last()
                .unwrap()
                .1
                * i
        })
        .sum();

    println!("{:?}", part1);

    let part2 = (0..6)
        .map(|j| {
            (1..=40)
                .map(|i| {
                    states
                        .clone()
                        .into_iter()
                        .filter(|s| s.0 == i + j * 40 || s.0 == i + j * 40 - 1)
                        .last()
                        .unwrap()
                        .1
                        - (i - 1)
                })
                .map(|dist| if dist.abs() <= 1 { "#" } else { "." })
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", part2);
    (0..5).map(|x| print!("{}", x)).rev().nth(3);
}
