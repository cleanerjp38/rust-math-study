use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut volume = 0;
    let mut sound = false;

    for _ in 0..q {
        let checker: u32 = iter.next().unwrap().parse().unwrap();
        
        match checker {
            1 => volume += 1,
            2 => {
                if volume >= 1 {
                    volume -= 1;
                }
            }
            3 => sound = !sound,
            _ => (),
        }
        if volume >= 3 && sound == true {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
//0162_abc442_B_2回目