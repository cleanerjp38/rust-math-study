use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut all_jems: Vec<Vec<u64>> = vec![Vec::new(); n];


    for _ in 0..n {
        let idx_c: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: u64 = iter.next().unwrap().parse().unwrap();
        all_jems[idx_c].push(v);

    }

    let mut colors_max: Vec<u64> = Vec::new();//各宝石の種類ごとの最大値を入れる配列
    let mut c_max: Vec<u64> = Vec::new();//C全体の最大値をk個分入れる配列
    let mut others: Vec<u64> = Vec::new();//その他の宝石を入れる配列

    for mut row in all_jems {
        if row.is_empty() {
            continue;
        }
        row.sort();
        row.reverse();
        colors_max.push(row[0]);
        
        //map()内で配列の中身を書き換えるのは非推奨らしい
        //そもそもmap()が最後では動作しない。collect()とかforみたいな実行させるものがないと何もしない
        //row.iter().map(|&i| others.push(i));
        others.extend_from_slice(&row[1..]);
    }

    colors_max.sort();
    colors_max.reverse();
    //extend_from_slice()は配列と配列を結合させる。便利過ぎる
    c_max.extend_from_slice(&colors_max[0..m]);
    others.extend_from_slice(&colors_max[m..]);
    
    others.sort();
    others.reverse();
    c_max.extend_from_slice(&others[0..k - m]);

    //ここで型をu64と指定し忘れていて、sum()のところでコンパイルエラーが出た
    //なんの型で返すのかでエラーということだった
    let result: u64 = c_max
        .iter()
        .sum();
    println!("{}", result);
}
//0149_abc461_C

//各宝石の種類の最大値を集めた配列colors_maxを作る
//sort()、reverse()を使って、colors_maxから大きい順にM個抜き出す
//color_maxとothersを結合させて、また大きい順にk-m個抜き出す
//これで各宝石の種類をM種類とり、かつ大きい順に抜き出せているので、合計して出力する