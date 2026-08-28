use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut horses: Vec<(u32, usize)> = Vec::with_capacity(n);
    for i in 1..=n {
        let t:u32 = iter.next().unwrap().parse().unwrap();
        horses.push((t, i));
    }

    horses.sort();
    println!("{} {} {}", horses[0].1, horses[1].1, horses[2].1);
}
//0164_2_abc440_B_タプル
//Vecへのタプルの詰め方と、出力方法
//sort()だと要素の1番目のほうでソートがかかる