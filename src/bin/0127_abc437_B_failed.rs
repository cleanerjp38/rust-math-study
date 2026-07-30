//h,w,n,a_vec,b_vecを一気に受け取る
//h行w列の2次元配列を作る
//1行ずつ調べていく
//この1次元配列の中に、Bと同じ数値があるか探す
//これをn回繰り返す
//前回のカウントと今回のカウントの大きい方を比較する
//それをh回繰り返す
//残ったカウント数を出力する

use std::io;

fn get_hwn() -> (usize, usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut hwn = input.trim().split_whitespace();
    let h: usize = hwn.next().unwrap().parse().unwrap();
    let w: usize = hwn.next().unwrap().parse().unwrap();
    let n: usize = hwn.next().unwrap().parse().unwrap();

    (h, w, n)
}

fn matrix_2d() -> u32 {
    let (h,w,n) = get_hwn();
    //let matrix:[[u32; w]; h] = [[0; w]; h];
    //let matrix: Vec<Vec<u32>> = vec![Vec::new(); h]; 配列の要素を入れ忘れた
    let mut matrix: Vec<Vec<u32>> =Vec::new();
    
    for _ in 0..h {
       let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let stdin = io::stdin();
    let mut max_ops = u32::MIN;

    for i in matrix.iter() {
        let mut current_ops: u32 = 0;
        //これではh×n行受け取ってしまう。このコードは間違い
        for _ in 0..n{
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let b: u32 = buf.trim().parse().unwrap();
            //current_ops += i.iter().filter(|&&item| item == b).count(); count()の戻り値はusize
            current_ops += i.iter().filter(|&&item| item == b).count() as u32;
        }
        if current_ops > max_ops {
            max_ops = current_ops;
        }
    }
    max_ops
}

fn main() {
    println!("{}", matrix_2d());
}
//0127_abc437_B_failed