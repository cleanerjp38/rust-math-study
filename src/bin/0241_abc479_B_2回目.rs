use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut count = 0;
    let mut max_count = 0;
    for c in s {
        if c == 'R' {
            count += 1;
        }

        if c == 'S'  {
            max_count = max_count.max(count);
            count = 0;
        }
    }

    max_count = max_count.max(count);

    println!("{max_count}");
}
//0241_abc479_B_2回目
// sについて全探索
// Rのとき、count += 1
// Sのとき、maxcount.max(count)
// RRR -> forループの外で、maxcountに数値を詰める作業が必要
// SSS -> countとmaxcountの初期値が0なのでOK
// RRS -> OK 
// RSR -> OK RRRの処理と同じ