use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s: String = input.trim().to_string();
    let s_len = s.len();

    s.remove(s_len / 2);

    println!("{}", s);
}
//0208_abc427_A_2回目