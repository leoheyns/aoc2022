use std::fs;

pub fn run(){
    let input = fs::read_to_string("src/day1/input").expect("invalid input");
    let elves= input.split("\n\n");
    let mut elve_totals = elves.map(|elve| elve.split("\n").map(|l| l.parse::<i32>().unwrap())).map(|e| e.sum::<i32>()).collect::<Vec<_>>();
    elve_totals.sort();
    println!("{}", elve_totals.iter().rev().take(3).sum::<i32>())

}