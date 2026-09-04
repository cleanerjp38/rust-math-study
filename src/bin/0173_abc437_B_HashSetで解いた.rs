use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut a_vec = Vec::with_capacity(h);

    for _ in 0..h {
        let mut row = Vec::with_capacity(w);
        for _ in 0..w {
            let a: u32 = iter.next().unwrap().parse().unwrap();
            row.push(a);
        }
        a_vec.push(row);
    }

    //let mut b_vec: Vec<u32> = Vec::with_capacity(n);
    //ここをVecでなくHashSetにすると、O(1)で検索できるそうだ
    //HashSetの仕組みがわからん。要練習だ
    let mut b_set: HashSet<u32> = HashSet::with_capacity(n);

    for _ in 0..n {
        let b: u32 = iter.next().unwrap().parse().unwrap();
        b_set.insert(b);
    }

    let mut max_count = 0;
    for row in a_vec {
        let current_max = row.iter().filter(|&b| b_set.contains(b)).count();
        max_count = max_count.max(current_max);
    }

    println!("{}", max_count);
}
//0173_abc437_B_HashSetで解いた
//AIは「a_vecを配列にしなくても、都度読み込みながらb_Vecと参照すればより高速になる」と言っていたが、標準入力の順番上不可能だった
//AIのロジックの間違いを見つけられたのは、今回が初かもしれん