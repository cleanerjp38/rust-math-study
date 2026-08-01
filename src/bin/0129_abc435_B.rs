//Nを受け取る。Aを配列で受け取る
//始めのインデックスをl，終わりのインデックスをrとする
//a.[l..=r].iter()で範囲内の数値を抜き出す
//部分列.iter().sum() % 部分列のiter() ==0になったらカウント
//カウントを出力する
use std::io;

fn get_ranges() -> (usize, Vec<u32>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<u32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    (n, a)
}

fn solve() -> u32 {
    let (n, a) = get_ranges();
    
    let mut count = 0;
    for l in 0..n {
        //ここの書き方を悩んだ。rはlとnの間にあるからこう書く
        for r in l..n {
            let slice = &a[l..=r];
            let sum: u32 = slice.iter().sum();

            //all()は、()内の条件が全てtrueであるかを返す
            //なので、is_okにはbool値が入る
            let is_ok = slice.iter().all(|&x| sum % x != 0);
            if is_ok {
                count += 1;
            }
        }
    }
    count 
} 

fn main() {
    println!("{}", solve());
}
//0129_abc435_B