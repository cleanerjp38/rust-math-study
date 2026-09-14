use std::io::{self, Read};

fn main() {
    let mut input =String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    //split_whitespace()はイテレータを作る、と初めて知った
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a_vec: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let a: i32 = iter.next().unwrap().parse().unwrap();
        a_vec.push(a);
    }
    //こうも書ける
    //let a_vec: Vec<i32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    for i in 0..n {
        let mut ans: i32 = -1;
        for j in (0..i).rev() {
            if a_vec[i] < a_vec[j] {
                ans = j as i32 + 1;
                break;
            }
        }
        println!("{}", ans);
    }
}
//0184_abc433_B
//nとA_vecを受け取る 
//for i in 0..n { 　
//for j in (0..i).rev() {
//でiの近くから探して、自分より背の高い人を見つけたらbreak、次へ向かう 
//ansはfor i の中で都度出力