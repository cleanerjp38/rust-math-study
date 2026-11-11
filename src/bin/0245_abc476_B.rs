use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    for i in 0..n {
        if s[i] == t[i] || t[i] == '*'  {
            continue;
        }

        if s[i] != t[i] {
            println!("No");
            return;
        }
    }

        //     if s[i] != t[i] && t[i] != '*'  {
        //     println!("No");
        //     return;
        // }
        //これで十分らしい…気付かなかった

    println!("Yes");
}
//0245_abc476_B
//s[i] == t[i] || t[i] == '*' なら次
//s[i] != t[i] なら、終わり
//最後まで通ったら、Yes
//先に'*'でcontinueしないと、returnしちゃう