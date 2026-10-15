use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut fishbones = Vec::with_capacity(n);

    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        fishbones.push((a, b));
    }

    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut names = Vec::with_capacity(m);

    for _ in 0..m {
        let name: String = iter.next().unwrap().parse().unwrap();
        names.push(name);
    }

    let mut available = HashSet::new();

    //for name in names {　これだとnamesを消費してしまう
    for name in &names {
        let len  = name.chars().count();
        for (i, c) in name.chars().enumerate() {
            //available.insert((len, i, c));
            available.insert((len, i + 1, c)); // i+1にしておくと、インデックスを1からスタートに出来る
        }
    }

    for name in &names {
        if name.len() != n {// 脊椎に相当する単語の長さが一致していなければ、そもそもアウト
            println!("No");
            continue;
        }

        let mut ok = true;
        
        for (i, c) in name.chars().enumerate() {
            let (a, b) = fishbones[i];

            if !available.contains(&(a, b, c)) {// fishboneに書いてある文字が、availableの文字と一致しない場合、false。
                ok = false;
            }
        }
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
//0214_abc452_C
//AIに教えてもらいながらだったので、自分でロジックを理解し切っている訳では無い