// use itertools::sorted;
use std::cmp::max;
use std::cmp::min;

pub fn _run() {
    let part1: i32 = include_str!("input")
        .lines()
        .map(|l| {
            l.split(&[',', '-'][..])
                .map(|s: &str| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|l: Vec<i32>| {
            if ((l[0] >= l[2]) && (l[1] <= l[3])) || ((l[0] <= l[2]) && (l[1] >= l[3])) {
                1
            } else {
                0
            }
        })
        .sum();
    let part2: i32 = include_str!("input")
        .lines()
        .map(|l| {
            l.split(&[',', '-'][..])
                .map(|s: &str| s.parse::<i32>().unwrap())
                .collect()
        })
        .map(|l: Vec<i32>| {
            if max(l[0], l[2]) <= min(l[1], l[3]) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", part1);
    println!("{}", part2);
}
