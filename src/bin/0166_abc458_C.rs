use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    //ここで改行を排除していなかったから、回答が0になってしまっていた。
    //1行の入力でも、trim()はいるのか
    let s = s.trim();
    let n: usize = s.len();
    let mut count = 0;
    let s_vec: Vec<char> = s.chars().collect();

    for i in 0..n {
        let left = i;
        let right = n - i - 1;
        if s_vec[i] == 'C' {
            count += left.min(right)  + 1;
        }
    }
    println!("{}", count);
}
//0166_abc458_C
//一歩ずつ進んでいって、Cを探す
//Cから見た左端か右端の小さいほうの数値が、部分文字列の個数となる
//それをカウントして、足していく