use std::io::{self, BufRead};

fn get_vec() -> u32 {
    let stdin= io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().expect("1").expect("2");
    let n: usize =first_line.trim().parse().expect("3");

    let mut a_vec: Vec<Vec<u32>> = Vec::new();
    for _ in 0..n {
        let second_line = lines.next().expect("4").expect("5");
        let mut a_line = second_line.trim().split_whitespace();
        a_line.next();//ここでLを捨てる
        let row: Vec<u32> = a_line.map(|s| s.parse().expect("6")).collect();
        a_vec.push(row);
    }

    //fn a_xy()でエラーになったので、こっちにコードを追記した
    let third_line = lines.next().expect("8").expect("9");
    let mut xy_line = third_line.trim().split_whitespace();
    let x: usize = xy_line.next().expect("10").parse().expect("11");
    let y: usize = xy_line.next().expect("12").parse().expect("13");

    //let result = a_vec[x][y];
    //xyは1からスタートだが、配列は0からスタートなので、1を引く
    let result = a_vec[x - 1][y- 1];
    result
}

fn main() {
    println!("{}", get_vec());
}
//0134_abc457_B_2回目