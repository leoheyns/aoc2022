pub fn _run() {
    let lines = include_str!("input").lines();
    let lines_vec = lines.collect::<Vec<_>>();

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // let mut total = 0;
    // for line in lines_vec{
    //     let left = &line[..line.chars().count()/2];
    //     let right = &line[line.chars().count()/2..];

    //     let mut count = 1;
    //     for letter in letters.chars(){
    //         if left.contains(letter) && right.contains(letter){
    //             total += count;
    //         }
    //         count += 1
    //     }
    // }

    let mut total_2 = 0;
    for i in 0..lines_vec.len() / 3 {
        let mut count = 1;
        for letter in letters.chars() {
            if lines_vec[i * 3].contains(letter)
                && lines_vec[i * 3 + 1].contains(letter)
                && lines_vec[i * 3 + 2].contains(letter)
            {
                total_2 += count;
            }
            count += 1
        }
    }

    // println!("{}", total);
    println!("{}", total_2)
}
