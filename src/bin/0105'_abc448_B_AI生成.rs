use std::io::{self, BufRead};

// 【名詞】Meal（食事） -> MealOrder（注文データ）
// ただの食事ではなく「システムに対する要求」であることを明確化
#[derive(Debug)]
struct MealOrder {
    pepper_id: usize,       // 種類 -> ID（システム上の識別子）
    requested_amount: u32,  // 上限 -> 顧客からの要求量 (Demand)
}

// 【名詞】Restaurant -> RestaurantInventory（レストランの在庫管理システム）
struct RestaurantInventory {
    num_pepper_types: u32,       // m -> 種類の総数
    stock_amounts: Vec<u32>,     // pepper_vec -> 店の在庫量リスト
    orders: Vec<MealOrder>,      // meals -> 注文リスト
}

impl RestaurantInventory {
    // 【関所】get_pepper -> load_from_stdin（標準入力からの読み込みと初期化）
    fn load_from_stdin() -> Self {
        let stdin = io::stdin();
        let buf = io::BufReader::new(stdin.lock());
        let mut lines = buf.lines();
        
        let first_line = lines.next().unwrap().unwrap();
        let mut nm = first_line.trim().split_whitespace();
        let num_orders: usize = nm.next().unwrap().parse().unwrap();
        let num_pepper_types: u32 = nm.next().unwrap().parse().unwrap();

        let second_line = lines.next().unwrap().unwrap();
        let stock_amounts: Vec<u32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut orders: Vec<MealOrder> = Vec::with_capacity(num_orders);
        for _ in 0..num_orders {
            let line = lines.next().unwrap().unwrap();
            let mut row = line.split_whitespace();
            let order = MealOrder {
                pepper_id: row.next().unwrap().parse::<usize>().unwrap() - 1,
                requested_amount: row.next().unwrap().parse().unwrap(),
            };
            orders.push(order);
        }

        // Self は RestaurantInventory 自身を指す
        Self { num_pepper_types, stock_amounts, orders }
    }

    // 【動詞】use_pepper -> aggregate_demands（要求量の集計）
    // 「使う」という曖昧な表現から「集計する(aggregate)」というIT用語へ
    fn aggregate_demands(&self) -> Vec<u32> {
        // pepper_sum -> total_demands（要求の合計）
        let mut total_demands: Vec<u32> = vec![0; self.num_pepper_types as usize];
        
        for order in &self.orders {
            total_demands[order.pepper_id] += order.requested_amount;
        }
        total_demands
    }

    // 【動詞】summary_pepper -> compute_actual_usage（実際の消費可能量の算出）
    // 単なる要約ではなく「計算して実態を出す(compute)」処理
    fn compute_actual_usage(&self, total_demands: Vec<u32>) -> u32 {
        // summary -> actual_usage（実際の使用量）
        let mut actual_usage: u32 = 0;
        
        for i in 0..(self.num_pepper_types as usize) {
            match total_demands[i] <= self.stock_amounts[i] {
                true => actual_usage += total_demands[i],
                false => actual_usage += self.stock_amounts[i],
            }
        }
        actual_usage
    }
}

fn main() {
    // インスタンス（実体）の名前も r ではなく inventory に
    let inventory = RestaurantInventory::load_from_stdin();
    
    // 処理の流れを英語の文章のように読めるようにする
    let demands = inventory.aggregate_demands();
    let max_consumption = inventory.compute_actual_usage(demands);
    
    println!("{}", max_consumption);
}
//0105'_abc448_B_AI生成