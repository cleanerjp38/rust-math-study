use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut h = vec![0; n];
    let mut count_h = vec![0; q];
    let mut cleared = 0;

    for _ in 0..q {
        let query_type: usize = iter.next().unwrap().parse().unwrap();

        if query_type == 1 {
            let x: usize = iter.next().unwrap().parse().unwrap();
            h[x] += 1;
            count_h[h[x]] += 1;
            
            if count_h[cleared + 1] == n {
                cleared += 1;
            }
        } else {
            let y: usize = iter.next().unwrap().parse().unwrap();
            let target = y + cleared;
            //↓では、y + cleared > qのときに対応できないそうだ。これ、要るのか？
            //println!("{}", count_h[target]);
            if y + cleared > q {
                println!("{}", 0);
            } else {
                println!("{}", count_h[target]);
            }
        }
    }
}
//0165_abc459_C
//---俺が考えたロジック---
//count＝1
//VecのNを作る
//forでQ回回す
//matchで分ける
//1のとき、インデックスxのマスに1を足す
//any(|i| !vec.contain(count))
//falseなら、count+=1
//2のとき、y-(count-1)以上のブロックを、filter().count()で数えて出力

//---AIが答えたロジックの俺の解釈---
//x軸のcount_hと、y軸のhを用意する
//count_h[h[x]]で、縦に積まれた箱の横1列の個数がわかる
//count_h[cleared + 1] == n、つまり、横一列がn個埋まったら、基準点のclearedを上げる
//yが来たら、count_h[y + cleared]を出力。clearedよりy個上へ伸びているhの個数を出力できる。

