//----そもそもロジックの崩壊したコードになってしまった----

use std::io::{self, BufRead};


fn get_nmst() -> (u32, u32, String, String) {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nm = first_line.trim().split_whitespace();
    let n: u32 = nm.next().unwrap().parse().unwrap();
    let m: u32 = nm.next().unwrap().parse().unwrap();
    let s = lines.next().unwrap().unwrap().trim().to_string();
    let t = lines.next().unwrap().unwrap().trim().to_string();

    (n, m, s, t)
}

//fn substring(s_short:&str, t:&str) -> u32 {
//ここでtを消費している。↑が正解
fn substring(s_short:String, t:String) -> u32 {
    let s_vec:Vec<char> = s_short.chars().collect();
    let t_vec:Vec<char> = t.chars().collect();

    let mut count:u32 = 0;
    for (idx, i) in s_vec.iter().enumerate() {
        let s_num: u32 = i.to_digit(10).unwrap();
        let t_num: u32 = t_vec[idx].to_digit(10).unwrap();

        if s_num < t_num {
            count += s_num - t_num + 10;
        } else {
            count += s_num - t_num;
        }
    }
    count
} 

//fn cut_string(m:u32, s:&str) -> String {
//ここでもsを消費している。↑が正解
//cut_string関数が、Sの先頭からM文字目までを切り取る処理になってしまっている
fn cut_string(m:u32, s:String) -> String {
    let s_vec: Vec<char> = s.chars().collect();

    let mut s_short: String = String::new();
    for i in 0..(m as usize){
        s_short.push(s_vec[i]);
    }
    s_short
}

fn main() {
    let (n, m, s, t) = get_nmst();
    let mut sum = 0;

    for _ in 0..(n as usize) {
        //ここでsとtの所有権が引っかかってる
        let s_short = cut_string(m, s);
        let count = substring(s_short, t);
        //以下が正解
        //let s_short = cut_string(m, &s);
        //let count = substring(s_short, &t);

        //sumで合計しちゃっているので、そもそも最小値の検出でない
        sum += count;
    }
    println!("{}", sum);
}
//0125_abc438_B_failed