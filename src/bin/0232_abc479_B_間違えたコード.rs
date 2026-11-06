use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<char> = iter.next().unwrap().to_string().chars().collect();

    let mut max_count = 0;
    let mut right = 0;
    let mut left = 0;

    while right < s.len() {

        match s[right] {
            'S' => {//反例　RRRR
                let current_count = right - left - 1;//反例　RRRSS
                left = right;
                max_count = max_count.max(current_count);
            }
            _ => (),
        }

        right += 1;
    }
    println!("{max_count}");
}
//0232_abc479_B_間違えたコード