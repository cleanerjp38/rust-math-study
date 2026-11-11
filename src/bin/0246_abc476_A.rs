use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s_char:Vec<char> = s.chars().collect();
    let mut ans = String::new();
    if s_char[s_char.len() - 1] == 'e' {
        let s_str: String = s_char.iter().collect();
        ans = format!("{}{}", s_str, 'r');
    } else {
        let s_str: String = s_char.iter().collect();
        ans = format!("{}{}", s_str, "er");
    }


    println!("{ans}");
}
//0246_abc476_A
//文字列の末尾を判定する方法が、charの配列に分解する以外に思いつかなかった
//end_with() chars().last() s.push(r) s.push_str(er) といったやり方もあるらしい