use std::io::{self, BufRead};

fn s_t_q() -> (String, String, u32) {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    lines.next().unwrap().unwrap();

    //ここ、sとtが1行で来ると勘違いしていた
    //let second_line = lines.next().unwrap().unwrap();
    //let mut st = second_line.split_whitespace();
    let s = lines.next().unwrap().unwrap().trim().to_string();
    let t = lines.next().unwrap().unwrap().trim().to_string();

    //(*c)で参照を消さないとエラーになった。なんでだろう？
    let u: String = s.chars().filter(|c| t.contains(*c)).collect();
    //let s_pure = s.chars().filter(|&c| !u.contains(c)).collect(); |&c|でもいいらしい
    let s_pure = s.chars().filter(|c| !u.contains(*c)).collect();
    let t_pure = t.chars().filter(|c| !u.contains(*c)).collect();

    let q: u32 =lines.next().unwrap().unwrap().trim().parse().unwrap();


    (s_pure, t_pure, q)

}

fn check(s_pure: String, t_pure: String, q: u32) {
    //forの外で標準入力をlock()しておくと早くなる
    //let stdin = io::stdin();
    //let mut handle = stdin.lock(); ループの外でロック

    for _ in 0..q {
        //今回は回数が少ないから良いが、for内での標準入力の受け付けは速度低下に繋がる
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let w = input.trim();

        //any()は、集合の中のいずれかの要素を持っているかどうかをboolで判定する
        let is_takahashi = w.chars().any(|c| s_pure.contains(c));
        let is_aoki = w.chars().any(|c| t_pure.contains(c));

        if is_takahashi {
            println!("Takahashi");
        } else if is_aoki {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}

fn main() {
    let (s, t, q) = s_t_q();
    check(s, t, q);
}
//0120_abc441_B