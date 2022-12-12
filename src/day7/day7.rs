use std::collections::HashMap;
pub fn _run(){
    let lines = include_str!("input").lines();
    let mut dirstack: Vec<String> = Vec::new();
    let mut dirsizes = HashMap::new();
    for line in lines{
        match line.split(" ").collect::<Vec<&str>>()[..]{
            ["$", "cd", ".."] => {dirstack.pop();},
            ["$", "cd", dir] => dirstack.push(dirstack.last().unwrap_or(&"".to_string()).to_owned() + "-" + dir),
            ["$", "ls"] => {},
            ["dir", _] => {},
            [amount, _] => {for dir in dirstack.clone(){dirsizes.entry(dir).and_modify(|s| *s += amount.parse::<i32>().unwrap()).or_insert( amount.parse::<i32>().unwrap());}},
            _ => {},
        }
    }
    let part1:i32 = dirsizes.values().filter(|s| **s <= 100000).sum();
    println!("{}", part1);
    let needed = dirsizes.get("-/").unwrap() - 40000000;
    let part2 = dirsizes.values().filter(|s| **s >= needed).min().unwrap();
    println!("{}", part2);
}