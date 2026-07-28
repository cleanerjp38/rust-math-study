use std::io::{self, BufRead};
use std::cmp;

fn get_nmst() -> (usize, usize, String, String) {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nm = first_line.trim().split_whitespace();
    let n:usize = nm.next().unwrap().parse().unwrap();
    let m:usize = nm.next().unwrap().parse().unwrap();
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();

    (n, m, s, t)
}

fn substring() -> u32 {
    let (n, m, s, t) = get_nmst();
    
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    //u32::MAXでその型の最大値を入れられる
    //最大値を初期値にすることで、変数を準備しておく
    let mut min_ops = u32::MAX;
    //ここ、0..(n - m)にしてた
    //0..=(n - m)をforで回すのは、B問題やC問題でよくあるらしい
    for i in 0..=(n - m) {
        let mut current_ops  = 0;
        for j in 0..m {
            let s_num: u32 = s_vec[j + i].to_digit(10).unwrap();
            let t_num: u32 = t_vec[j].to_digit(10).unwrap();

            //%10で余りを出す。モジュラーかあ、俺じゃまだまだ自力で書けないなあ
            let diff = (s_num + 10 - t_num) % 10;
            current_ops += diff;
        }
        //cmp::min()で最小値の比較ができる
        //cmpってなんの略だろう？
        min_ops = cmp::min(min_ops, current_ops);
    }
    min_ops
}

fn main() {
    println!("{}", substring());
}
//0125_2_abc438_B

//N、M、S、Tを受け取る
//SをTと同じ長さに一部分を切り取る ここからforとか使う
//S'の配列とTの配列の同じインデックス番号をチェックする
//S1=9, T1=0の場合、9-0=9で、9を加算
//S1=0, T1=9の場合、1を加算するはず。なので、0-9+10=1
//これらの数をcountに加算していく
//この動作をSのT文字分の部分文字列を全て検証する　回す回数はs-tかな？
//最小値を見つける
//なので、count_vecを作って、配列の中の最小値を探す？
//見つけた数値を出力する
