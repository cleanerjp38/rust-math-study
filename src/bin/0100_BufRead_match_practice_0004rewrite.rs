use std::io::{self, BufRead};

struct NumberPair {
    a: u32,
    b: u32,
}

fn get_numbers() -> NumberPair {
    let buf = io::BufReader::new(io::stdin());

    //ここで止まった。lines()からどうやって数値を取り出すのだろうか？
    //parse()もsplit_whitspace()もない。あるのはnext()くらいだった。
    //let inputs = buf.lines()
    
    //ここでnext()は行を送っているのだろう。で、入力が空かどうかをexpectでチェックする
    let line_result = buf.lines().next().expect("INPUT NULL");
    //またexpectで、今度は行の読み込みチェックをする。それをなんでexpect()単体でできるんだ？
    //あ、よく見たらアナライザーの型推論に「Result<String, Error>」って書いてある。すげー
    let line = line_result.expect("INPUT Error");
    //let line = buf.lines().next().expect("INPUT NULL").expect("INPUT Error"); これでもできた

    //なんでここのinputsはmutじゃないといけないんだろう？next()で中の要素を排出しているから、inputsも編集しているのか？
    let mut inputs = line.split_whitespace();
    let a: u32 = inputs.next().unwrap().parse().unwrap();
    let b: u32 = inputs.next().unwrap().parse().unwrap();

    NumberPair { a, b }
}

impl NumberPair {
    fn check_parity(&self) -> String {
        let product = self.a * self.b;

        //ifの代わりにmatchを使うのも良い練習になるね
        //数値の場合は''で囲わなくて良い
        match product % 2 {
            0 => String::from("Even"),
            _ => String::from("Odd"),
        }
    }
}

fn main() {
    let pair = get_numbers();
    let result = pair.check_parity();
    println!("{}", result);
}
//0100_BufRead_match_practice_0004rewrite