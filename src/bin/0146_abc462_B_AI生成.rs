use std::io::{self, Read};

//ロジックはなんとなく頭に浮かんでいても、実際にこう綺麗に書けなかった。
//やはり頭の中だけで構造を置いてコードを書くのは、まだ俺には難しいのかもしれない
//----以下はAIが提示したコード----
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    
    //ここのvec![Vec::new(); n + 1]の書き方を思いつけなかった。こう書いていいんだな
    let mut receivers: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in 1..=n {
        let k: usize = iter.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let a: usize = iter.next().unwrap().parse().unwrap();
            receivers[a].push(i);
        }
    }

    for i in 1..=n {
        let x = receivers[i].len();
        print!("{}", x);
        
        for sender in &receivers[i] {
            print!(" {}", sender);
        }
        println!();
    }
}
//0146_abc462_B_AI生成