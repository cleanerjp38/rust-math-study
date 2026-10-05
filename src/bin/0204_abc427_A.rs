use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    let mut s: Vec<char> = input.chars().collect();
    let s_len = s.len();
    //let middle = (s_len + 1 / 2);
    let middle = s_len / 2;

    s.remove(middle);
    //let ans = format!("{:?}", s);
    let ans: String = s.iter().collect();

    println!("{}", ans);
}
//0204_abc427_A