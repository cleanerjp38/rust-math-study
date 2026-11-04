use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let abcd: Vec<char> = input.trim().split_whitespace().flat_map(|line| line.chars()).collect();

    let a = abcd.iter().position(|&c| c == 'A').unwrap();
    let d = abcd.iter().position(|&c| c == 'D').unwrap();

    if a + d == 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
//0229_abc480_A