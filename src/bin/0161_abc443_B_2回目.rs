use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let k: u32 = iter.next().unwrap().parse().unwrap();

    let mut current_mame = n;
    let mut sum = n;
    let mut count  = 0;

    while sum < k {
        current_mame += 1;
        sum += current_mame;
        count += 1;
    }

    println!("{}", count);
}
//0161_abc443_B_2回目
//簡単に書けた