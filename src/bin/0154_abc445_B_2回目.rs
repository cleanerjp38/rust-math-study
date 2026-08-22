use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut s_vec: Vec<String> = Vec::with_capacity(n);
    let mut max: usize = 0;
    for _ in 0..n {
        let s: String = iter.next().unwrap().parse().unwrap();
        max = max.max(s.len());
        s_vec.push(s);
    }

    for t in s_vec {
        //usizeは万一マイナスになるとエラーになるので、max - (t.len() + max) / 2という書き方をしている
        let k = max - (t.len() + max) / 2;
        //repeat()は文字列だけでなく、数値でもリピートするのだろうか？
        let dots = ".".repeat(k);
        println!("{}{}{}", dots, t, dots);
    }
}
//0154_abc445_B_2回目
//repeat()の使い方を調べた以外は、さっくり書けた