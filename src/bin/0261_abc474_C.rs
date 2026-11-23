use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [u32; n],
        a: [u32; q],
    }

    let mut hashset = HashSet::new();
    let mut querry_vec = Vec::new();

    for &j in a.iter().rev() {
        if hashset.insert(j) {
            querry_vec.push(j)
        }
    }

    let mut remove_p: Vec<&u32> = p.iter().filter(|&x| !hashset.contains(x)).collect();
    querry_vec.reverse();

    for i in &querry_vec {
        remove_p.push(i);
    }

    for ans in remove_p {
        print!("{ans} ");
    }
}
//0261_abc474_C

// クエリが末尾に入っていくので、クエリをHashSetにまとめる
// PをそのHashSetでフィルターして、被っているものを除外する
// PとはHashSetをガッチャンコ
// クエリが（1→2→1）のとき、HashSetに（2,1）の順で積められるかどうか