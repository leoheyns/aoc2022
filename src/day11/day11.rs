use std::collections::HashMap;

pub fn _run() {
    let monkey_strings = include_str!("input").split("\n\n");

    struct Monkey {
        items: Vec<i64>,
        operation: Box<dyn Fn(i64) -> i64>,
        condition_target: Box<dyn Fn(i64) -> usize>,
        activity: i32,
    }

    let mut monkeys: Vec<Monkey> = monkey_strings
        .map(|ms| {
            ms.split("\n")
                .map(|l| l.split(": ").last().unwrap())
                .collect::<Vec<&str>>()
        })
        .map(|mss| Monkey {
            items: mss[1]
                .split(", ")
                .map(|it| it.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operation: {
                match mss[2].split(" ").collect::<Vec<&str>>()[..] {
                    ["new", "=", "old", "*", "old"] => Box::new(|old| old * old / 3),
                    ["new", "=", "old", "*", val] => {
                        Box::new(|old| old * val.parse::<i64>().unwrap() / 3)
                    }
                    ["new", "=", "old", "+", "old"] => Box::new(|old| (old + old) / 3),
                    ["new", "=", "old", "+", val] => {
                        Box::new(|old| (old + val.parse::<i64>().unwrap()) / 3)
                    }
                    _ => Box::new(|old| old),
                }
            },
            condition_target: {
                match mss[3..=5]
                    .iter()
                    .map(|l| l.split(" ").last().unwrap())
                    .collect::<Vec<&str>>()[..]
                {
                    [divider, true_monke, false_monke] => Box::new(|worry| {
                        if worry % divider.parse::<i64>().unwrap() == 0 {
                            true_monke.parse::<usize>().unwrap()
                        } else {
                            false_monke.parse::<usize>().unwrap()
                        }
                    }),
                    _ => Box::new(|_| 0),
                }
            },
            activity: 0,
        })
        .collect();

    let mut thrownitems = HashMap::new();

    for _ in 0..20 {
        for (i, monke) in monkeys.iter_mut().enumerate() {
            thrownitems
                .entry(i)
                .and_modify(|items: &mut Vec<i64>| monke.items.extend(items.drain(..)));
            for item in monke.items.drain(..) {
                monke.activity += 1;
                let worry = (monke.operation)(item);
                thrownitems
                    .entry((monke.condition_target)(worry))
                    .and_modify(|items: &mut Vec<i64>| items.push(worry))
                    .or_insert(vec![worry]);
                // monkeys[(monke.condition_target)(worry)].items.push(worry);
                // println!("{} {}", i, (monke.condition_target)(worry))
            }
        }
    }
    println!(
        "{:?}",
        monkeys.iter().map(|m| m.activity).collect::<Vec<i32>>()
    );
    _part2();
}

fn _part2() {
    let monkey_strings = include_str!("input").split("\n\n");

    struct Monkey {
        items: Vec<i64>,
        operation: Box<dyn Fn(i64) -> i64>,
        condition_target: Box<dyn Fn(i64) -> usize>,
        activity: i64,
    }

    let mut monkeys: Vec<Monkey> = monkey_strings
        .map(|ms| {
            ms.split("\n")
                .map(|l| l.split(": ").last().unwrap())
                .collect::<Vec<&str>>()
        })
        .map(|mss| Monkey {
            items: mss[1]
                .split(", ")
                .map(|it| it.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operation: {
                match mss[2].split(" ").collect::<Vec<&str>>()[..] {
                    ["new", "=", "old", "*", "old"] => {
                        Box::new(|old| (old * old) % (5 * 11 * 2 * 13 * 7 * 3 * 17 * 19))
                    }
                    ["new", "=", "old", "*", val] => Box::new(|old| {
                        (old * val.parse::<i64>().unwrap()) % (5 * 11 * 2 * 13 * 7 * 3 * 17 * 19)
                    }),
                    ["new", "=", "old", "+", "old"] => {
                        Box::new(|old| (old + old) % (5 * 11 * 2 * 13 * 7 * 3 * 17 * 19))
                    }
                    ["new", "=", "old", "+", val] => Box::new(|old| {
                        (old + val.parse::<i64>().unwrap()) % (5 * 11 * 2 * 13 * 7 * 3 * 17 * 19)
                    }),
                    _ => Box::new(|old| old),
                }
            },
            condition_target: {
                match mss[3..=5]
                    .iter()
                    .map(|l| l.split(" ").last().unwrap())
                    .collect::<Vec<&str>>()[..]
                {
                    [divider, true_monke, false_monke] => Box::new(|worry| {
                        if worry % divider.parse::<i64>().unwrap() == 0 {
                            true_monke.parse::<usize>().unwrap()
                        } else {
                            false_monke.parse::<usize>().unwrap()
                        }
                    }),
                    _ => Box::new(|_| 0),
                }
            },
            activity: 0,
        })
        .collect();

    let mut thrownitems = HashMap::new();

    for _ in 0..10000 {
        for (i, monke) in monkeys.iter_mut().enumerate() {
            thrownitems
                .entry(i)
                .and_modify(|items: &mut Vec<i64>| monke.items.extend(items.drain(..)));
            for item in monke.items.drain(..) {
                monke.activity += 1;
                let worry = (monke.operation)(item);
                thrownitems
                    .entry((monke.condition_target)(worry))
                    .and_modify(|items: &mut Vec<i64>| items.push(worry))
                    .or_insert(vec![worry]);
                // monkeys[(monke.condition_target)(worry)].items.push(worry);
                // println!("{} {}", i, (monke.condition_target)(worry))
            }
        }
    }
    let mut activities = monkeys.iter().map(|m| m.activity).collect::<Vec<i64>>();
    activities.sort_unstable();
    let result: i64 = activities.iter().rev().take(2).product();

    println!("{:?}", result)
}
