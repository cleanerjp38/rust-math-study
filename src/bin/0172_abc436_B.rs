use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut square = vec![vec![0; n]; n];
    let mut r = 0;
    let mut c = (n - 1) / 2;
    let mut k = 1;
    square[r][c] = k;

    for _ in 0..((n * n) - 1) {
        k += 1;
        //(r - 1)だと、rがusizeなので最初の処理が-1になってしまってパニックになる。
        //if square[(r - 1) % n][(c + 1) % n] == 0 {
            //r = (r - 1) % n;
        if square[(r + n - 1) % n][(c + 1) % n] == 0 {
            r = (r + n - 1) % n;
            c = (c + 1) % n;
            square[r][c] = k;
        } else {
            r = (r + 1) % n;
            square[r][c] = k;
        }
    }

    for row in square {
        for i in row {
            print!("{} ", i);
            //println!(""); ここだと1文字ずつ改行になる
        }
        println!("");
    }
}
//0172_abc436_B
//最初にN×Nの0の配列を作っておく
//あとは問題文の指示通りに数式を書いた
//usizeを使うときは引き算に注意。以前もなにかの問題で間違えた