use std::io;
use std::collections::HashSet;

fn formula(n: u32) -> u32 {
    let mut sum = 0;
    for c in n.to_string().chars() {
        let x = c.to_digit(10).unwrap();
        sum += x * x;
    }

    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    let mut visited:HashSet<u32> = HashSet::new();

    loop {
        let sum = formula(n);
        match sum {
            1 => {
                println!("Yes");
                return;
            }
            _ if visited.contains(&sum) => {
                println!("No");
                return;
            }
            _ => {
                visited.insert(sum);
                n = sum;
            }
        }
    }
}
//0202_abc439_B_3回目