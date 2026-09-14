use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut x: Vec<char> = input.trim().chars().collect();
    //charもソートできると知らなかった
    x.sort();

    if x[0] == '0' {
        for i in 1..x.len() {
            if x[i] != '0' {
                //swap(a,b)で、インデックスaとbの要素を入れ替える
                x.swap(0, i);
                break;
            }
        }
    }
    
    let ans: String = x.into_iter().collect();
    println!("{}", ans);
}
//0185_abc432_B
//数字を Vec<char> にする
//昇順ソート
//先頭が 0 なら、最初の 0 でない文字を探す
//それと先頭を swap
//そのまま String に戻す