use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut dots_vec = Vec::new();

    for c in s {
        if c != 'A' {
            dots_vec.push('.');
        } else {
            dots_vec.push(c);
        }
    }

    let ans: String = dots_vec.iter().collect();

    println!("{ans}");
}
//0258_abc472_A

// 新しい配列に、A以外は.を入れる
// その配列をStringに変換して出力