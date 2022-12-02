pub fn run(){
    let input = include_str!("input");
    println!("{}", input.split("\n").map(|l|l.split(" ").map(to_shape).collect::<Vec<i32>>()).map(|r| part1(r[0], r[1])).sum::<i32>());
    println!("{}", input.split("\n").map(|l|l.split(" ").map(to_shape).collect::<Vec<i32>>()).map(|r| part2(r[0], r[1])).sum::<i32>());

}

fn to_shape(letter:&str) -> i32{
    match letter{
        "A" | "X" => return 0,
        "B" | "Y" => return 1,
        "C" | "Z" => return 2,
        _ => return 3,
    }
}

fn part1(p1:i32, p2:i32) -> i32{
    if (p1 + 1) % 3 == p2{
        //p2 wint
        return 7 + p2
    } else if p1 == p2 {
        //draw
        return 4 + p2
    } else {
        //p1 wint
        return 1 + p2
    }
}

fn part2(p1:i32, p2:i32) -> i32{

    match p2 {
        0 => return 1 + (p1 + 2) % 3,
        1 => return 4 + p1,
        2 => return 7 + (p1 + 1) % 3,
        _ => return 0
    }
}