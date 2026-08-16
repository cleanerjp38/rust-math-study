use std::io::{self, Read};
//cmpを書かなくてもmax()はあったけど、min()はなかったぞ？
use std::cmp::min;

fn pepper() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut c_vec: Vec<u64> = Vec::with_capacity(m);
    for _ in 0..m {
        c_vec.push(iter.next().unwrap().parse().unwrap());
    }
    
    //ここは[0u64]と書かないとコンパイルエラーとなった
    //胡椒の料理に対する使用量をVecにまとめておく
    let mut sum_vec =vec![0u64; m];

    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: u64 = iter.next().unwrap().parse().unwrap();
        //インデックスは0始まりなのでa-1
        sum_vec[a - 1] += b;
    }

    let mut count = 0;

    for i in 0..m {
        //count += c[j].min(req_sum[j]);でも良いらしい。その場合はcmp::minが要らない
        //出来る限りstd::cmpのような不要な関数は使わず、メソッドで書くほうが良いらしい
        count += min(c_vec[i], sum_vec[i]);
    }

    println!("{}", count);
}

fn main() {
    pepper();
}
//0145_abc448_B_2回目