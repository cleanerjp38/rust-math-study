use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

        let mut b = Vec::with_capacity(m);
    for _ in 0..m {
        b.push(iter.next().unwrap().parse::<u32>().unwrap());
    }

    a.sort();
    b.sort();

    let mut count = 0;
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if b[j] <= a[i] * 2 {
            count += 1;
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }

    println!("{}", count);
}
//0156_abc460_C

//AiとBiを配列に詰める。2つとも小さい順にソートする
//NかMの小さいほうの数値でforを回す
//if A[0] * 2 => b[i] {
//  count += 1;
//  a[0].remove;
//  a[0].remove;}
//と書こうとしたが、それだとオーバーフローするらしい
//尺取り法を教えてもらい、そのコードを練習した