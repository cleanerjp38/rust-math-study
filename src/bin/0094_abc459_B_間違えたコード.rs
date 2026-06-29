//----以下は間違えたコード------

use std::io::{self, Read};

fn get_strings() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.split_whitespace();
    //ここでNを捨てているのに、forのところでは捨てていない前提でコードを書いている
    tokens.next().unwrap();
    let strings_vec: Vec<String> = tokens.map(|s| s.parse().unwrap()).collect();

    strings_vec
}
enum Initial {
    Abc2,
    Def3,
    Ghi4,
    Jkl5,
    Mno6,
    Pqrs7,
    Tuv8,
    Wxyz9,
    None,
}

impl Initial {
    //check()で&selfを入れていない。これではenumの要素が参照できない
    //しかも、Vec<>を指定していない
    //fn check(&self) -> Vec<String> {
    fn check() -> String {
        let mut strings_vec = get_strings();
        let mut ans_vec =Vec::new();

        //最初のNを捨てているのに、&strings_vec[1..]と入れて、さらに1文字スキップしている
        //for s in &strings_vec[0..] {
        for s in &strings_vec[1..] {
            let first_char = s.chars().next().unwrap();

            let initial_room = match first_char {
                'a' | 'b' | 'c' => Initial::Abc2,
                'd' | 'e' | 'f' => Initial::Def3,
                'g' | 'h' | 'i' => Initial::Ghi4,
                'j' | 'k' | 'l' => Initial::Jkl5,
                'm' | 'n' | 'o' => Initial::Mno6,
                'p' | 'q' | 'r' | 's' => Initial::Pqrs7,
                't' | 'u' | 'v' => Initial::Tuv8,
                'w' | 'x' | 'y' | 'z' => Initial::Wxyz9,
                _               => Initial::None,
            };

            let Initial_num = match initial_room {
                Initial::Abc2 => '2'.to_string(),
                Initial::Def3 => '3'.to_string(),
                Initial::Ghi4 => '4'.to_string(),
                Initial::Jkl5 => '5'.to_string(),
                Initial::Mno6 => '6'.to_string(),
                Initial::Pqrs7 => '7'.to_string(),
                Initial::Tuv8 => '8'.to_string(),
                Initial::Wxyz9 => '9'.to_string(),
                Initial::None => "".to_string(),
            };
            ans_vec.push(Initial_num);
        }
        //返り値を書いていない
        //ans_vec 
    }
}

fn main() {
    
}
//0094_abc459_B_間違えたコード