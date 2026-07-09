use std::io::{self, BufRead};

struct Restaurant {
    m: u32,
    pepper_vec: Vec<u32>,
    meals: Vec<Vec<u32>>,
}

fn get_pepper() -> Restaurant {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nm = first_line.trim().split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: u32 = nm.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    //コショウ一つ一つの最大容量
    let pepper_vec: Vec<u32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    //食事とコショウの配列
    let mut meals: Vec<Vec<u32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        meals.push(row);
    }

    Restaurant { m, pepper_vec, meals }
}

impl Restaurant {
    fn use_pepper(&self) -> Vec<u32> {
        //vec![]でm個要素が入る一次配列を作る
        let mut pepper_sum: Vec<u32> = vec![0; self.m as usize];
        for meals_vec in &self.meals {
            //料理番号を一つ取り出す
            let meal_a = meals_vec[0];
            //料理番号をインデックスにする（料理番号は1スタートなので、1引いている）
            let index = (meal_a - 1) as usize;
            //料理に使うコショウの量を取り出す
            let pepper_b = meals_vec[1];
            //料理番号に対してどれだけコショウを使うか足していく
            pepper_sum[index] += pepper_b;
        }
        pepper_sum
    }

    fn summary_pepper(&self, pepper_sum: Vec<u32>) -> u32 {
        //使ったコショウの合計値
        let mut summary:u32 = 0;
        //mがu32なので、usizeに変換する必要があった
        //for i in 0..self.m {
        for i in 0..(self.m as usize) {
            match pepper_sum[i] <= self.pepper_vec[i] {
                //Ok()はResult型につかうらしい。よくわかってない
                //Ok() => {
                true => summary += pepper_sum[i],
                //Err() => {
                false=> summary += self.pepper_vec[i],
            }
        }
        summary
    }
}

fn main() {
    let r = get_pepper();
    println!("{}", r.summary_pepper(r.use_pepper()));
}
//0105_abc448_B