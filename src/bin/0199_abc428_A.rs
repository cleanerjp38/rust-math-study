use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let s: u32 = iter.next().unwrap().parse().unwrap();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let mut x: u32 = iter.next().unwrap().parse().unwrap();

    let mut sum = 0;

    'time: loop {
        let mut count_a = 0;
        let mut count_b = 0;
        while count_a < a {
            sum += s;
            count_a += 1;
            x -= 1;
            if x == 0 {
                break 'time;
            }
        }
        while count_b < b {
            count_b += 1;
            x -= 1;
            if x == 0 {
                break 'time;
            }
        }
    }
    println!("{}", sum);
}
//0199_abc428_A