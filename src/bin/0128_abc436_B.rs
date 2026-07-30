//nを受け取る
//gird:vec<vec<u32>>を準備する
//grid[0][(n-1)/2].insert(1)が出来るか検証
//for _ in 0..((n*n) -1)
//r=0,c=(n-1)/2,k=1を初期値にする
//if grid[(r-1)%n][(c+1)%n]==NULLまたは_のとき、k+1をinsert()
//そうでない場合、grid[(r+1)%n][c]にk+1を入れる
//gridを出力する
//イテレータで流して出力できそうだが、やり方がわからん

use std::io;

fn insert_grid() -> Vec<Vec<i32>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    //let mut grid: Vec<Vec<i32>> = Vec::new();
    //判定をするために配列に0を入れておくと良い
    let mut grid :Vec<Vec<i32>> = vec![vec![0;n];n];
    let mut r = 0;
    let mut c = (n -1) / 2;
    let mut k = 1;
    grid[r][c] = k;

    for _ in 0..((n * n) - 1) {
        k += 1;
        let next_r = (r + n - 1) % n;
        let next_c = (c + 1) % n;
        //NULLはRustにはないので、0で判定する
        //if grid[(r - 1) % n][(c + 1) % n] == NULL {
        if grid[next_r][next_c] == 0 {
            //grid[next_r][next_c] = k;
            //next_r, next_cの数値を先に入れ替えたほうが良い
            r = next_r;
            c = next_c;
        } else {
            //grid[(r + 1) % n][c] = k;
            r = (r + 1) % n;
        }
        grid[r][c] = k;
    }

    grid
}

fn main() {
    let result =insert_grid();

    for row in result {
        let line: Vec<String>  = row.iter().map(|&x| x.to_string()).collect();
        println!("{}", line.join(" "));
        //↓の書き方だと、[]や,も一緒に出力されてしまう
        //println!("{:?}", i);
    }
}
//0128_abc436_B