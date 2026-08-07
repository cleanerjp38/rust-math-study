use std::io;

fn get_ns() -> (usize, String) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut ns = input.trim().split_whitespace();
    let n: usize = ns.next().unwrap().parse().unwrap();
    let s: String = ns.next().unwrap().parse().unwrap();
    (n, s)
}

//kの回数分xを引いたら終了
//k回目にxを引いたら、その手前までのoまで袋を開けたことになる
//よって、k-1
//xよりkが大きかったら、nの最後まで袋を開けられる。
fn count_x() {
    let (n, s) = get_ns();
    let mut x_position = Vec::new();

    for (idx, c) in s.chars().enumerate() {
        //xの位置の把握
        if c == 'x' {
            x_position.push(idx + 1);
        }
    }

    for k in 1..=n {
        //xの個数がkより大きかった場合
        if k <= x_position.len() {
            //あれ、ここがk-1である論理がわからなくなったぞ
            println!("{}", x_position[k - 1]);
        } else {
            println!("{}", n);
        }
    }
}

fn main() {
    count_x();
}
//0135_abc469_C_AI生成_読解