use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a_vec: Vec<u32> = Vec::with_capacity(n);

    for _ in 0..n {
        let a: u32 = iter.next().unwrap().parse().unwrap();
        a_vec.push(a);
    }

    let mut count = 0;
    for l in 0..n - 1 {
        let mut sum = 0;
        for r in l..n {
            let mut check = true;
            sum += a_vec[r];
            for i in l..=r {
                if sum % a_vec[i] as u32 == 0 {
                    check = false;
                    break;
                }
            }
            if check {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
//0175_abc435_B