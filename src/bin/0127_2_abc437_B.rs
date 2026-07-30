//h,w,nを受け取る
    //aをfor in 0..h で作る
    //bをfor in 0..n で作る
//for in a.iter()で1行ずつ調べていく
//この1次元配列の中に、Bと同じ数値があるものをカウントする
//max()で最大値を探す
//最大値を出力する

use std::io::{self, BufRead};
use std::cmp;

fn get_grids() -> (Vec<Vec<u32>>, Vec<u32>) {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut hwn = first_line.trim().split_whitespace();
    let h: usize = hwn.next().unwrap().parse().unwrap();
    let w: usize = hwn.next().unwrap().parse().unwrap();
    let n: usize = hwn.next().unwrap().parse().unwrap();

    let mut a_vec: Vec<Vec<u32>> = Vec::new();
    for _ in 0..h {
        //ここでline変数を作らないと、linesが消費されてしまって借用チェッカーに引っかかる
        let line = lines.next().unwrap().unwrap();
        let a_line = line.trim().split_whitespace();
        let row = a_line.map(|s| s.parse().unwrap()).collect();
        a_vec.push(row);
    }

    let mut b_vec: Vec<u32> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let b: u32 = line.trim().parse().unwrap();
        b_vec.push(b);
    }

    (a_vec, b_vec)
}

fn count_num() -> u32 {
    let (a_vec,  b_vec) = get_grids();
    let mut max_ops = 0u32;

    for i in a_vec.iter() {
        let current_ops = i.iter().filter(|&b| b_vec.contains(b)).count();
        //数値を代入しないで捨ててた、アホか！
        //max_ops.max(current_ops as u32);
        max_ops = cmp::max(max_ops, current_ops as u32);
    }
    max_ops
}

fn main() {
    println!("{}", count_num());
}
//0127_2_abc437_B