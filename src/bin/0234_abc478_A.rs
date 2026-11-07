use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut grape: u32 = iter.next().unwrap().parse().unwrap();

    let mut people = vec![0u32; n];
    'count: while grape > 0 {
        for idx in 0..n {
            people[idx] += 1;
            grape -= 1;

            if grape == 0 {
                continue 'count;
            }
        }
    }

    for ans in people {
        println!("{}", ans);
    }
}
//0234_abc478_A