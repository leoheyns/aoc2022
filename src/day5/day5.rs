pub fn _run() {
    let input_split: Vec<&str> = include_str!("input").split("\n\n").collect();
    let init = input_split[0].split("\n").collect::<Vec<&str>>();
    let init_rev = init.iter().rev();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let instructions = input_split[1]
        .split("\n")
        .map(|l| l.split(" ").collect::<Vec<&str>>());

    for line in init_rev {
        let l_chars: Vec<char> = line.chars().collect();
        for i in 0..9 {
            if l_chars[i * 4 + 1].is_alphabetic() {
                stacks[i].push(l_chars[i * 4 + 1])
            }
        }
    }
    for instruction in instructions {
        // for _i in 0..instruction[1].parse::<i32>().unwrap(){
        //     let temp = stacks[instruction[3].parse::<usize>().unwrap() - 1].pop().unwrap();
        //     stacks[instruction[5].parse::<usize>().unwrap() - 1].push(temp)
        // }
        let amount = instruction[1].parse::<usize>().unwrap();
        let from = instruction[3].parse::<usize>().unwrap() - 1;
        let to = instruction[5].parse::<usize>().unwrap() - 1;
        let fromlen = stacks[from].len();
        let slice = &stacks[from].clone()[fromlen - amount..];
        stacks[to].extend_from_slice(slice);
        stacks[from].truncate(fromlen - amount)
    }
    for stack in stacks {
        print!("{}", stack[stack.len() - 1])
    }
    println!("")
}
