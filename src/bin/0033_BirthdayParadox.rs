use std::io;

fn get_input() -> i32 {
    let mut input_n = String::new();

    io::stdin()
        .read_line(&mut input_n)
        .expect("読み込み失敗");

    let n: i32 = input_n.trim().parse().unwrap();

    n
}

struct BirthdayParadox {
    days_in_year: f32,
    people: i32,
}

impl BirthdayParadox {
    fn new() -> Self {
        Self {
            days_in_year: 365.0,
            people: get_input(),
        }
    }

    fn prob_all_different(&self) -> f32 {
        let mut probability = 1.0;

        for i in 0..self.people {
            probability *=
                (self.days_in_year - i as f32) / self.days_in_year;
        }

        eprintln!("{}", probability);
        probability
    }
}

fn main() {
    let simulator = BirthdayParadox::new();

    let prob_diff = simulator.prob_all_different();
    eprintln!("{}", prob_diff);

    let prob_same = (1.0 - prob_diff) * 100.0;
    eprintln!("{}", prob_same);

    println!(
        "{}人の中に同じ誕生日がいるのは",
        simulator.people
    );
    println!("{}%です", prob_same);
}
//0033_BirthdayParadox