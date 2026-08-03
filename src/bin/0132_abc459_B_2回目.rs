use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let result: String = input
        .split_whitespace()
        .skip(1)
        //filter()は「条件に合致したものを次に流していいか」というbool判定を、以後の処理に求める
        //.filter(|&word| word.chars().next())
        //filter_map()は「Optionを受け取り、NoneならスキップしてSomeならその中身を取り出す」
        .filter_map(|word| word.chars().next())
        .map(|c| match c {
            'a'..='c' => '2',
            'd'..='f' => '3',
            'g'..='i' => '4',
            'j'..='l' => '5',
            'm'..='o' => '6',
            'p'..='s' => '7',
            't'..='v' => '8',
            'w'..='z' => '9',
            _ => unreachable!(),
        })
        .collect();

    println!("{}", result);
}
//0132_abc459_B_2回目