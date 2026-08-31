use std::io;

fn formula(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let i = n % 10;
        sum += i * i;
        n /= 10;
    }
    sum 
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();

    let mut sum_vec: Vec<u32> = Vec::new();
    loop {
        let sum = formula(n);

        match sum {
            1 => {
                println!("Yes");
                break;
            }
            _ if sum_vec.contains(&sum) => {
                println!("No");
                break;
            }
            _ => {
                sum_vec.push(sum);
                n = sum;
            }
        }
    }
}
//0167_abc439_B_2回目