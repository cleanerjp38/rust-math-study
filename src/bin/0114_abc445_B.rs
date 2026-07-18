use std::io::{self, BufRead};

struct Alignment {
    n: u32,
    string_vec: Vec<String>,
}

impl Alignment {
    fn get_vec() -> Self {
        let std = io::stdin();
        let buf = io::BufReader::new(std.lock());
        let mut lines = buf.lines();
        let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let mut string_vec:Vec<String> = Vec::new();
        for _ in 0..n {
            let next_line = lines.next().unwrap().unwrap();
            let string_line = next_line.trim().to_string();
            string_vec.push(string_line);
        }

        Self { n, string_vec }
    }

    fn count_char(&self) -> usize {
        let mut max = 0;
        for i in &self.string_vec {
            if max <= i.len() {
                max = i.len();
            } 
        }
        max
    }

    fn output_strings(&self, max: usize) {
        //単純な計算式を間違えた
        //let dot = max / 2;
        for s in &self.string_vec {
            let k = (max - s.len()) / 2;
            //AIにrepeat()という機能を教えてもらった。
            //おそらく"."ならString、"1"ならi32みたいに型ごと作ってくれるのだろう
            let dots = ".".repeat(k);
            //printlnで出力しても良かったが、折角なので文字列同士を一つにする方法を知りたかった
            //println!("{}{}{}", dots, s, dots);
            //format!()はいいね、使い方がprintln!()とそっくりで理解しやすい
            let combined = format!("{}{}{}", dots, s, dots);
            println!("{}", combined);
        }

    } 
}

fn main() {
    let alignment = Alignment::get_vec();
    let max = alignment.count_char();
    alignment.output_strings(max);
}
//0114_abc445_B