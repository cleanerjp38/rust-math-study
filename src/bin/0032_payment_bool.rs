use std::io;

fn get_input() -> (i32, i32) {
    let mut input_n = String::new();
    io::stdin()
        .read_line(&mut input_n)
        .expect("Nの読み込み失敗");

    let n: i32 = input_n
        .trim()
        .parse()
        .unwrap();

    let mut input_a = String::new();
    io::stdin()
        .read_line(&mut input_a)
        .expect("Aの読み込みに失敗");

    let a: i32 = input_a
        .trim()
        .parse()
        .unwrap();

    (n, a)
}

struct PaymentSystem {
    target_amount: i32,
    one_yen_coins: i32,
}

impl PaymentSystem {
    fn new() -> Self {
        let (n, a) = get_input();

        PaymentSystem {
            target_amount: n,
            one_yen_coins: a,
        }
    }

    fn can_pay(&self) -> bool {
        let change = self.target_amount % 500;
        change <= self.one_yen_coins
    }
}

fn main() {
    let system = PaymentSystem::new();

    if system.can_pay() {
        println!("Yes");
    } else {
        println!("No");
    }
}
//0032_payment_bool