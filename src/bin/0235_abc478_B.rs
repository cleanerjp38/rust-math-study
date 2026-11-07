use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let v: usize = iter.next().unwrap().parse().unwrap();
    let happiness: Vec<u64> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut max_happiness = 0;

    for a in 1..=n {//この三重ループすら思いつかなかった。基本ができていない。
        for b in (a + 1)..=n {
            for c in (b + 1)..=n {

                if a + b + c <= v {
                    let current_happiness = happiness[a - 1] + happiness[b - 1] + happiness[c - 1];
                    max_happiness = max_happiness.max(current_happiness);
                }
            }
        }
    }

    println!("{max_happiness}");
}
//0235_abc478_B
//3種類を選ぶ問題を、三重ループに変換する発想がまだ定着していなかった