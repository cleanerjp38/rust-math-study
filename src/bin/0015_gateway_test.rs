mod gateway;
use gateway::Gateway;

fn main() {
    let mut gate = Gateway::new();
    let n: i32 = gate.next();
    println!("読み込んだ数字は: {}", n);
}
//0015_gateway_test

//Runしたら、ターミナルをクリックして、適当な数字（例：123）を打ち込んで Enter を押し、その後に以下のショートカットを叩いてみて。
//Windowsの場合: Ctrl + Z を押してから Enter
//Mac/Linuxの場合: Ctrl + D
//0015_gateway_test


//----以下はgatewayモジュールのコピペ----

use std::io::{self, Read};
use std::collections::VecDeque;

pub struct Gateway{
    tokens: VecDeque<String>
}

impl Gateway {
    pub fn new() -> Self {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("読み込めなかったよ");
        let tokens: VecDeque<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();

        // Rustは最後にセミコロンを付けないことで「これを返す」という意味になる
        Self{tokens}
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.tokens
            .pop_front()              // 1. アイテムボックスから一個出す (Option型)
            .expect("入力がもうないよ") // 2. 中身を強制的に取り出す（空なら怒る）
            .parse::<T>()             // 3. 望みの型に変換する (Result型)
            .ok()                     // 4. 変換成功なら値を出し、失敗ならNoneにする
            .expect("型変換に失敗したよ") // 5. 最終的な値を取り出す
    }
}