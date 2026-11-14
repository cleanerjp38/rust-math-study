use std::io;

fn get_input() -> u32 {
    let mut input_n = String::new();

    io::stdin()
        .read_line(&mut input_n)
        .expect("Error");

    let n: u32 = input_n.trim().parse().unwrap();

    n
}

struct Food {
    n: u32,
}

impl Food {
    fn new() -> Self {
        Self {
            n: get_input(),
        }
    }

    fn caliculate(&self) -> u32 {
        800 * self.n - (self.n / 15) * 200
    }
}

fn main() {
    let food = Food::new();
    let result = food.caliculate();

    println!("合計支払額は{}円です", result);
}
//0034_Summary_ABC_solved