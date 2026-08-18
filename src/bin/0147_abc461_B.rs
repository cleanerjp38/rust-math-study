use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a_row: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        a_row.push(iter.next().unwrap().parse::<usize>().unwrap() - 1);
    }
    let mut b_row : Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        b_row.push(iter.next().unwrap().parse::<usize>().unwrap() - 1);
    }

    for i in 0..n {
        let a = a_row[i];
        let b = b_row[a];
        if i != b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
//0147_abc461_B

//配列AのAiの要素の値をaとする
//配列BのBaの要素の値をbとする
//このとき、i!=bなら、No 
//forで回して、i≠bが一人でもいたら、Noを出力してreturnで関数から抜ける
//全員満たしていたら、forを抜けてから、YESを出力
//入力ではインデックスは1スタートなので、Vecを作る際に要素の値から1を引いて、0-indexedにしておく