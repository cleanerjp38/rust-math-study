use std::io;

fn get_case_data() -> (i64, i64, i64, i64, i64, i64) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.split_whitespace();
    //もっと上手い書き方はあるかもしれない。array[]をfor6つ並べて回していくとか
    //けど、ならこの書き方でforを回す回数を減らしたほうが良いのでは？
    let x1: i64 = tokens.next().unwrap().parse().unwrap();
    let y1: i64 = tokens.next().unwrap().parse().unwrap();
    let r1: i64 = tokens.next().unwrap().parse().unwrap();
    let x2: i64 = tokens.next().unwrap().parse().unwrap();
    let y2: i64 = tokens.next().unwrap().parse().unwrap();
    let r2: i64 = tokens.next().unwrap().parse().unwrap();

    (x1, y1, r1, x2, y2, r2)
}

fn kyouyuuten() {
    let mut t_buffer = String::new();
    io::stdin().read_line(&mut t_buffer).unwrap();
    let t: i64 = t_buffer.trim().parse().unwrap();

    for _ in 0..t {
        //ここで、入力もforで回しているから、ターミナルに入力と出力が1行ずつ交互に書かれている。
        //これは、回答として成立しているのか？
        let (x1, y1, r1, x2, y2, r2) = get_case_data();
        //(x1-x2).pow(2)は正の整数しか駄目。pow()系は負の数には使えない？
        let x_m = x1 - x2;
        let y_m = y1 - y2;
        let r_m = r1 - r2;
        let r_p = r1 + r2;

        if (r_m * r_m) <= (x_m * x_m) + (y_m * y_m) && (x_m * x_m) + (y_m * y_m) <= (r_p * r_p) {
            println!("yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    kyouyuuten();
}
//0093_abc460_B_二回目