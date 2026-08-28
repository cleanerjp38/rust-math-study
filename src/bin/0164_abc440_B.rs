use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut t: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        t.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    let mut horses: Vec<usize> = (1..=n).collect();
    horses.sort_by_key(|&i| t[i - 1]);

    println!("{} {} {}", horses[0], horses[1], horses[2]);
}
//0164_abc440_B
//sort_by_key()の使い方を要練習
//タプルでやるやり方もあるらしいのでやってみる