use std::collections::HashSet;

pub fn _run() {
    let input: Vec<char> = include_str!("input").chars().collect();
    // let input:Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
    for i in 0..input.len() - 14 {
        if HashSet::<_>::from_iter(&input[i..i + 14]).len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}
