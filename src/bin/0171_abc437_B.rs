use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut a_vec: Vec<Vec<u32>> = vec![Vec::with_capacity(w); h];
    for _ in 0..h {
        let mut row = Vec::with_capacity(w);
        for _ in 0..w {
            let a_i: u32 = iter.next().unwrap().parse().unwrap();
            row.push(a_i);
        }
        a_vec.push(row);
    }

    let mut b_vec: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        let b: u32 = iter.next().unwrap().parse().unwrap();
        b_vec.push(b);
    }

    let mut max_count = 0;
    for row in a_vec {
        let current_max = row.iter().filter(|&b| b_vec.contains(b)).count();
        max_count = max_count.max(current_max);
    }

    println!("{}", max_count);
}
//0171_abc437_B
//Aの2次元配列とBの1次元配列を作る
//Aを1列ずつ見て、Bの配列と同じ数値をカウントする
//現在のcount数と最大countを比較して、大きい方を保持
//最大countを出力
