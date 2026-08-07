use std::io;

struct Asset {
    amount: f64,
}

impl Asset {
    fn pass_year(&mut self, rate: f64) {
        self.amount *= 1.0 + rate;
    }
}

fn get_years() -> u32 {
    println!("何年後の資産を知りたいですか？");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("泥が混入しました");
    input.trim().parse().unwrap_or(0)
}

fn main() {
    let years = get_years();
    let mut my_money = Asset { amount: 10000.0 };
    for year in 1..=years {
        my_money.pass_year(0.05);
        println!("{}年目:{:.0}円", year, my_money.amount);
    }
}
//0011_状態遷移のコード