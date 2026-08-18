use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut y_arr = vec![0; n];
    for _ in 0..n {
        let x: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;//ここで1を引き忘れてパニックになってた
        let y: u32 = iter.next().unwrap().parse().unwrap();
        y_arr[x] = y;
    }

    let mut count = 0;
    let mut min_y = u32::MAX;

    for i in 0..n {
        let current_y = y_arr[i];
        if min_y > current_y {
            count += 1;
        }
        min_y = min_y.min(current_y);
    }

    println!("{}",count);
}
//0148_abc462_C

//xを1～Nまで整列させる
//Xi＜Xjのとき、Yj<Yiであれば、問題の要求と合致する
//xの要素の値をインデックスとして、Yxを検証する
//for i in 0..n で回して、
//Yの最小値＞Yiのとき、count+=1

