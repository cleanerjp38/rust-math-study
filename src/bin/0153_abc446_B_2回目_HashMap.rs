use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter= input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    //let mut juice_map: HashMap<u32, usize> = HashMap::with_capacity(m);
    let mut juice_map: HashMap<u32, bool> = HashMap::with_capacity(m);

    for _ in 0..n {//このループ内で、客が何を飲むかを処理する
        let l: u32 = iter.next().unwrap().parse().unwrap();
        let mut chosen: u32 = 0;//客が飲むもの。初期値は0
        for _ in 0..l {
            let juice = iter.next().unwrap().parse().unwrap();
            //飲むものが0、かつ、指定したjuiceの判定がfalseの場合
            if chosen == 0 && !juice_map.contains_key(&juice) {
                chosen = juice;
                juice_map.insert(juice, true);
            }
        }
        //その客が飲むものを出力
        println!("{}", chosen);
    }
}
//0153_abc446_B_2回目_HashMap

//その客が最終的に飲むものを入れる変数（初期値は水 ＝ 0）を用意する。
//希望リストを前から順に見ていく。
//もし「まだ飲み物が決まっていない」かつ「そのジュースが HashMap（売約済みリスト）に存在しない」なら、それを選ぶ！
//選んだら、HashMapにそのジュースを「売約済み」として登録する。