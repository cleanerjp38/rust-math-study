use std::io;

struct Birthdays {
    x: u32,
    y: u32,
    z: u32,
}

impl Birthdays {
    fn check(&self) -> bool {
        for i in 0..100 {
            if (self.x + i) == (self.y + i) * self.z {
                return true;
            }
        }
        false
    }
}

fn get_numbers() -> Birthdays {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut inputs = buffer.split_whitespace();
    let x: u32 = inputs.next().unwrap().parse().unwrap();
    let y: u32 = inputs.next().unwrap().parse().unwrap();
    let z: u32 = inputs.next().unwrap().parse().unwrap();

    Birthdays { x, y, z }
}

fn main() {
    let birthdays = get_numbers();
    if birthdays.check() {
        println!("Yes");
    } else {
        println!("No");
    }
}
//0092_abc433_A_二回目