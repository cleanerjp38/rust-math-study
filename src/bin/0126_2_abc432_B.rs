//Xを受け取る
//xをVec＜u32＞にする　←間違い、Vec<char>でよかった
//xの配列を小さい順に並び替える
    //vec.sort()で配列内の要素を小さい順にする
//最初に0が並んでいる場合、その0を次にくる数値の直後に再配置する
//00012→10002みたいに
    //first_non_zero_idxが>0の場合
    //non_zero_charをremoveで引っ張り出して、digit[0]にinsertする
//値を出力する

use std::io;

fn get_x() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x_st = input.trim();
    let x_vec: Vec<char> = x_st.chars().collect();

    x_vec
}

fn sort_minimize() -> String {
    let mut x_vec = get_x();
    
    x_vec.sort();
    //1．ここが書けなかった。Someとposition()をまだ理解していない
    if let Some(first_non_zero_idx) = x_vec.iter().position(|&c| c != '0') {
        //remove()は、配列から要素を取り出して変数に入れるメソッドかな？
        let non_zero_char = x_vec.remove(first_non_zero_idx);
        //insert()は、対象の配列番号に指定された要素を割り込ませる
        x_vec.insert(0, non_zero_char);
    }

    //2.ここも書けなかった。iter().collect()でStringになるのか
    //じゃあ、数値にもなるのか？　←ならない。なぜならx_vecが&charの配列だから
    let result: String = x_vec.iter().collect();
    result
}

fn main() {
    println!("{}", sort_minimize());
}
//0126_2_abc432_B