use std::io;

fn get_input() -> (i32, i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 文字列を空白で分割し、整数に変換してVecへ集める
    let v: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Vecの各要素をタプルとして返す
    (v[0], v[1], v[2])
}

struct TravelPlan {
    people: i32,
    train_fee: i32,
    taxi_fee: i32,
}

impl TravelPlan {
    fn new() -> Self {
        let (n, a, b) = get_input();

        Self {
            people: n,
            train_fee: a,
            taxi_fee: b,
        }
    }

    fn min_cost(&self) -> i32 {
        let total_train = self.people * self.train_fee;
        let total_taxi = self.taxi_fee;

        std::cmp::min(total_train, total_taxi)
    }
}

fn main() {
    let travel_plan = TravelPlan::new();
    let result = travel_plan.min_cost();

    println!("{}", result);
}
//0035_TravelPlan_(Vec)