use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: Vec<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();
    let mut count = 0;

    // 'counter: for r in 1..n {
    //     let mut current_sum = 0;

    //     for l in 0..r {
    //         current_sum += a[l];
    //     }
    //     for i in 0..r {
    //         if current_sum % a[i] as u32 == 0 {
    //             continue 'counter;
    //         }
    //     }
    //     count += 1;
    // }

    for l in 0..n {
        'counter: for r in l..n {
            let current_sum: u32 = a[l..=r].iter().sum();
            
            for i in l..=r {
                if current_sum % a[i] == 0 {
                    continue 'counter;
                }
            }
            count += 1;
        }

    }

    println!("{}", count);
}
//0215_abc435_B_2回目