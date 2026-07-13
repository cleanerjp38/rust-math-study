use std::io::{self, BufRead};

/// 客一人のデータ（物語の名詞）
struct Customer {
    wish_list: Vec<u32>,
}

/// 配給のシミュレーション全体を管理する構造体
struct Simulation {
    customers: Vec<Customer>,
    total_juices: u32,
}

impl Simulation {
    /// Step 2: 入力という「泥」を捌き、純粋な型を返す関所（Gateway）
    fn from_stdin() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let first_line = lines.next().unwrap().unwrap();
        let mut nm = first_line.split_whitespace();
        let n: usize = nm.next().unwrap().parse().unwrap();
        let m: u32 = nm.next().unwrap().parse().unwrap();

        let mut customers = Vec::new();

        for _ in 0..n {
            // 客の希望個数（配列の長さから分かるため、読み飛ばすだけでOK）
            let _len_line = lines.next().unwrap().unwrap();

            // 客の希望リスト（X_i,j）
            let list_line = lines.next().unwrap().unwrap();
            let wish_list: Vec<u32> = list_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            customers.push(Customer { wish_list });
        }

        Self {
            customers,
            total_juices: m,
        }
    }

    /// Step 3: 物語の進行（シミュレーション実行）
    fn run(&self) {
        // ジュースが誰かに取られたかを記録する「チェックシート」
        // 1番〜M番をそのままインデックスとして扱うため、サイズは m + 1 にする
        // 初期状態はすべて false（まだ誰も取っていない）
        let mut is_taken = vec![false; (self.total_juices + 1) as usize];

        for customer in &self.customers {
            let mut got_juice = 0; // 何も得られなかった場合のデフォルトは水(0)

            // 自分の希望リストを先頭から順に確認する
            for &juice in &customer.wish_list {
                let j_idx = juice as usize;
                
                // もしそのジュースがまだ取られていなければ（falseなら）
                if !is_taken[j_idx] {
                    is_taken[j_idx] = true; // 「自分が取った」という状態(true)に書き換える
                    got_juice = juice;      // 獲得したジュースの番号を記録する
                    break;                  // 1本手に入れたら、それ以降の希望リストは見なくてよい
                }
            }

            // この客が最終的に得たもの（ジュースの番号、または水なら0）を出力する
            println!("{}", got_juice);
        }
    }
}

fn main() {
    // 1. 関所を通じて、100%信頼できる純粋なデータ構造を生成
    let sim = Simulation::from_stdin();
    
    // 2. 準備されたデータを使って、シミュレーションを実行
    sim.run();
}
//0111_2_abc446_B_AI生成