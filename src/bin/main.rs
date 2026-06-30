///-----AI生成コードに注釈をつけた-----
use std::io::{self, Read};

struct WordList {
    words: Vec<String>,
}

impl WordList {
    fn convert_to_numbers(&self) -> String {
        //こんな書き方ができるとは、想像もしていなかった。
        self.words//構造体の要素を使う
            .iter()//配列の中身を一つずつ「参照」する
            //map()は()の中のルールに沿って変数を変換する。ここではself.wordsを変換
            .map(|word|{//|word|は集合の要素の指定
                let first_char = word.chars().next().unwrap();
                //word.chars() wordは文字列でくるので、それを一文字ずつのイテレータに分解する
                //next() 分解された文字列のうち、最初の一文字を抜き出す
                //.unwrap() 空文字だったときのエラー（None）の想定を無視して、中身を保証する。空文字ならプログラムを強制終了する

                match first_char {
                    //文字も..=といういった表記をしていいのか！
                    'a'..='c' => '2',
                    'd'..='f' => '3',
                    'g'..='i' => '4',
                    'j'..='l' => '5',
                    'm'..='o' => '6',
                    'p'..='s' => '7',
                    't'..='v' => '8',
                    'w'..='z' => '9',
                    _         => ' ',//matchは厳格だから、その他の文字の処理もする
                }
            })
            //collect()はVecをつくるのかと思っていたが、「バラバラの要素」を一つにまとめるという機能
            //「-> String」としてあるので、Rustの型推論によってStringへとまとめられている
            .collect()
    }
}

fn get_word_list() -> WordList {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let words: Vec<String> = buffer
        .split_whitespace()
        //skip()の()内は捨てる個数。なので最初の1個を捨てたい場合はskip(0)番目でなくてskip(1)個目
        .skip(1)
        .map(|s| s.to_string())
        .collect();

    WordList { words }
}

fn main() {
    let word_list = get_word_list();
    println!("{}", word_list.convert_to_numbers());
}
//0095_abc459_B_AI生成_読解