use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    let mut right: usize = 0;
    let mut count: u64 = 0;
    let n: usize = input.len();
    let s: Vec<char> = input.chars().collect();

    for left in 0..n {
        if left > right {
            right = left;
        }

        //while right < n && s[left] != s[right] {
        while right < n && (right == left || s[right] != s[right - 1]) {
            right += 1;
        }
        count = (count + (right - left) as u64) % 998244353;
    }

    println!("{}", count);
}
//0176_abc456_C_ロジック作らず雑にやった