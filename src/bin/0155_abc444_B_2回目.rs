use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nk = input.trim().split_whitespace();
    let n: usize = nk.next().unwrap().parse().unwrap();
    let k: u32 = nk.next().unwrap().parse().unwrap();
    let mut count: u32 = 0;
    for i in 1..=n {
        let i_string: String = i.to_string();
        let mut sum: u32 = 0;
        for j in i_string.chars() {
            sum += j.to_digit(10).unwrap();
        }
        if sum == k {
            count += 1;
        }
    }

    println!("{}", count);
}
//0155_abc444_B_2回目

//1..=nで回して、nまでの全ての整数を確認する
//i.to_string()でiを文字列にして、chars()で1数値ずつ流す
//文字をto_digit(10)で数値に直して、足す
//sum == kならcountを1増やす
//コードも自分で書けた