use std::collections::HashSet;

pub fn _run() {
    let mut xs = vec![0; 10];
    let mut ys = vec![0; 10];
    let lines = include_str!("input").lines();
    let mut visited = HashSet::new();

    for line in lines {
        let instruction: Vec<&str> = line.split(" ").collect();
        for _ in 0..instruction[1].parse().unwrap() {
            match instruction[0] {
                "U" => ys[0] += 1,
                "D" => ys[0] -= 1,
                "L" => xs[0] -= 1,
                "R" => xs[0] += 1,
                _ => {}
            }

            for i in 0..9 {
                if ((xs[i] - xs[i + 1]) as i32).abs() > 1 && ((ys[i] - ys[i + 1]) as i32).abs() > 1
                {
                    xs[i + 1] = xs[i] + ((xs[i + 1] - xs[i]) as i32).signum();
                    ys[i + 1] = ys[i] + ((ys[i + 1] - ys[i]) as i32).signum();
                } else if ((xs[i] - xs[i + 1]) as i32).abs() > 1 {
                    ys[i + 1] = ys[i];
                    xs[i + 1] = xs[i] + ((xs[i + 1] - xs[i]) as i32).signum();
                } else if ((ys[i] - ys[i + 1]) as i32).abs() > 1 {
                    xs[i + 1] = xs[i];
                    ys[i + 1] = ys[i] + ((ys[i + 1] - ys[i]) as i32).signum();
                }
            }
            visited.insert((xs[9], ys[9]));
        }
    }
    println!("{}", visited.len());

    for i in (-20..20).rev() {
        for j in -20..20 {
            print!("{}", if visited.contains(&(j, i)) { "#" } else { "." })
        }
        println!()
    }
}
