use std::cmp::{max, min};
use std::collections::HashSet;

pub fn _run() {
    let input = include_str!("input")
        .split("\n")
        .map(|l| l.split(": ").collect::<Vec<_>>())
        .map(|l| {
            (
                l[0].split(" ").collect::<Vec<_>>(),
                l[1].split(" ").collect::<Vec<_>>(),
            )
        })
        .map(|s| {
            (
                (
                    s.0[2][2..(s.0[2].len() - 1)].parse::<i32>().unwrap(),
                    s.0[3][2..].parse::<i32>().unwrap(),
                ),
                (
                    s.1[4][2..(s.1[4].len() - 1)].parse::<i32>().unwrap(),
                    s.1[5][2..].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

    let l = 2000000;
    let mut ranges: Vec<(i32, i32)> = vec![];
    let mut beacons_on_line = HashSet::new();

    //part1
    for line in input.clone() {
        if line.1 .1 == l {
            beacons_on_line.insert(line.1);
        }

        let manhattan = (line.0 .0 - line.1 .0).abs() + ((line.0 .1 - line.1 .1) as i32).abs();
        let dist_to_l = ((line.0 .1 - l) as i32).abs();

        if dist_to_l <= manhattan {
            ranges.push((
                line.0 .0 - manhattan + dist_to_l,
                line.0 .0 + manhattan - dist_to_l + 1,
            ));
        }
    }

    let mut reduced_ranges: Vec<(i32, i32)> = vec![];

    ranges.sort_unstable();

    for r in ranges {
        let rlen = reduced_ranges.len().clone();
        if rlen > 0 {
            let current_max = reduced_ranges[rlen - 1].1;
            if current_max >= r.0 {
                if r.1 > current_max {
                    reduced_ranges[rlen - 1].1 = r.1
                }
            } else {
                reduced_ranges.push(r)
            }
        } else {
            reduced_ranges.push(r)
        }
    }

    let part1: i32 = reduced_ranges
        .iter()
        .map(|range| range.1 - range.0)
        .sum::<i32>()
        - (beacons_on_line.len() as i32);
    println!("{}", part1);

    for l in 0..4000000 {
        let mut ranges: Vec<(i32, i32)> = vec![];

        //part1
        for line in input.clone() {
            let manhattan = (line.0 .0 - line.1 .0).abs() + ((line.0 .1 - line.1 .1) as i32).abs();
            let dist_to_l = ((line.0 .1 - l) as i32).abs();

            if dist_to_l <= manhattan {
                ranges.push((
                    line.0 .0 - manhattan + dist_to_l,
                    line.0 .0 + manhattan - dist_to_l + 1,
                ));
            }
        }

        let mut reduced_ranges: Vec<(i32, i32)> = vec![];

        ranges.sort_unstable();

        for r in ranges.iter().map(|r| (max(0, r.0), min(4000000, r.1))) {
            let rlen = reduced_ranges.len().clone();
            if rlen > 0 {
                let current_max = reduced_ranges[rlen - 1].1;
                if current_max >= r.0 {
                    if r.1 > current_max {
                        reduced_ranges[rlen - 1].1 = r.1
                    }
                } else {
                    reduced_ranges.push(r)
                }
            } else {
                reduced_ranges.push(r)
            }
        }

        if reduced_ranges.len() != 1 {
            println!(
                "{}",
                (4000000 * (reduced_ranges[0].1 as i128)) + (l as i128)
            );
            break;
        }
    }
}
