use std::io;

struct NumberCards {
    cards: Vec<u32>,
}

impl NumberCards {
    fn form_stdin() -> Self {
        //1行の標準入力に対してBufReadを使う必要はなかった
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cards: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        NumberCards { cards }
    }

    fn form_max_number(&self) -> u32 {
        //ここでclone()を使う理由がよくわからない
        let mut sorted_cards = self.cards.clone();

        //sort()で小さい順に並べる
        sorted_cards.sort();
        //reverse()で後ろから順に変える
        sorted_cards.reverse();

        //配列を数値に変えたものを返す
        sorted_cards[0] * 100 + sorted_cards[1] * 10 + sorted_cards[2]
    }
}

fn main() {
    let cards = NumberCards::form_stdin();
    println!("{}", cards.form_max_number());
}
//0113_abc432_A_failed_AI生成


//----以下は俺が間違えたコード----

//use std::io::{self, BufRead};

//fn get_abc() -> (u32, u32, u32) {
    //let std = io::stdin();
    //let buf = io::BufReader::new(std.lock());
    //let mut lines = buf.lines();
    //let first_line = lines.next().expect("INPUT NULL").expect("INPUT Error");
    //let mut abc = first_line.trim().split_whitespace();
    //let a = abc.next().expect("INPUT NULL").parse::<u32>().expect("INPUT Error");
    //let b: u32 = abc.next().expect("INPUT NULL").parse().expect("INPUT Error");
    //let c: u32 = abc.next().expect("INPUT NULL").parse().expect("INPUT Error");

//    (a, b, c)
//}

//    fn combination(a:u32, b:u32, c:u32) {
//        let mut result: u32 = 0;
//        if a <= b {
//            if b <= c {
//                result = (100 * c) + (10 * b) + a;
//            } else if a <= c {
//                result = (100 * b) + (10 * c) + a;
//            } else {
//                result = (100 * c) + (10 * a) + c;
//            }
//        } else if a <= c {
//            result = (100 * c) + (10 * a) + b;
//            } else if b <= c {
//                result = (100 * a) + (10 * c) + b;
//            }  else {
//                result = (100 * a) + (10 * b) + c;
//            }
//        }
//        println!("{}", result);
//}


//fn main() {
//    let (a, b, c) = get_abc();
//    combination(a, b, c); 
//}