use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut trees: Vec<bool> = vec![false; 100000000];

    for _ in 0..q {
        let querry = iter.next().unwrap().parse::<u32>().unwrap() - 1;
        let tree: usize = iter.next().unwrap().parse().unwrap();

        match querry {
            1 =>  { 
                trees[tree] = true;
            }
            2 => {
                trees.iter().take(tree).filter(|&&t| t == true).map(|&t| t == false);//iter()は参照だけ。map()内の「t == false」は比較してるだけ
                //for t in trees.iter_mut().take(tree) {
                    // t = false;
                //};
            }
            _ => (),
        }
        let count = trees.iter().filter(|&&t| t == true).count();

        println!("{}", count);
    }
}
//0218_abc451_C_間違えたコード
//この書き方だと、計算量が膨大すぎる
//BinaryHeapを使うと良いらしい