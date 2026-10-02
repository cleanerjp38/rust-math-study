use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n:usize = iter.next().unwrap().parse().unwrap();
    let t: Vec<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut horses: Vec<usize> = (1..=n).collect();

    horses.sort_by_key(|i| t[i - 1]);
    println!("{} {} {}", horses[0], horses[1], horses[2]);
}
//0201_abc440_B_2回目