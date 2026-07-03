//BufReaderを使う時に宣言するトレイトBufRead
use std::io::{self, BufRead};

fn experiment_line() {
    //io::stdin()を変数にまとめて可読化
    let stdin = io::stdin();
    //lock()は、これから来る標準入力をreader変数以外で受け取れなくする
    //io::BufReader::new()でBufReader構造体を呼び出し、引数に標準入力を入れる
    //まだ変数宣言なので、標準入力を受け付けてはいない？
    let reader = io::BufReader::new(stdin.lock());

    println!("--- 文字を入力してEnterを押す（空行で終了） ---");

    //index:usize line:Resultと型推論されるのはなぜだ？←enumerate()によって通し番号というusizeが生まれている
    //lines()は1行読み込む、enumerate()はなんだろう？
    //enumerate()は「イテレータを流れる要素に通し番号を振る」メソッド
    //おそらく、ここで標準入力の受け付けが開始されている
    for (index, line) in reader.lines().enumerate() {
        //受け付けた入力を変数に入れている
        //unwrap()があるのは、入力受付時に接続エラーとかをコンパイラが警戒しているから、らしい。よくわからん
        let line_str = line.unwrap();
        //is_empty()はわかりやすい名前だね。こういうのもあるのか
        if line_str.is_empty() {
            //breakはfor構文から脱出
            break;
        }
        //1行分の文字数のカウント。iter()の後ろでなくてchars()の後ろでもcount()は使えるのか
        //chars()もイテレータを生み出しているから
        let char_count = line_str.chars().count();

        //「\"」はなんだ？←「"」を文字として認識させている
        println!("[{}行目] 入力: \"{}\" (文字数: {})", index + 1, line_str, char_count);
    }
}

fn main() {
    experiment_line();
}
//0099_BufRead_test_読解