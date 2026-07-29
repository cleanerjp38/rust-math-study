use std::io;

fn x() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x_str = input.trim();

    let digits: Vec<char> = x_str.chars().collect();

    digits
}

fn sort_min(mut digits: Vec<char>) -> String {
    digits.sort();
    if let Some(first_idx) = digits.iter().position(|&c| c != '0') {
        if first_idx > 0 {
            let non_zero_char = digits.remove(first_idx);
            digits.insert(0, non_zero_char);
        }
    }
    let result: String = digits.into_iter().collect();
    result
} 

fn main() {
    let digits = x();
    println!("{}", sort_min(digits));
}
//0126_abc432_B_練習



//Xを受け取る
//xをVec＜u32＞にする　←間違い、Vec<char>でよかった
//xの配列を小さい順に並び替える
    //vec.sort()で配列内の要素を小さい順にする
//最初に0が並んでいる場合、その0を次にくる数値の直後に再配置する
//00012→10002みたいに
    //first_non_zero_idxが>0の場合
    //non_zero_charをremoveで引っ張り出して、digit[0]にinsertする
//値を出力する
