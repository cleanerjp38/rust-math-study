use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<char> = iter.next().unwrap().to_string().chars().collect();

    let mut max_count = 0;
    
    let mut count = 0;
    for c in s {
        if c == 'R' {
            count += 1;
            max_count = max_count.max(count);//R側にこれを書かないと、「RRRR」がきたときに最大値を更新できない
        } else {
            count = 0;
        }
    }

    println!("{max_count}");
}
//0233_abc479_B