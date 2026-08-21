use std::collections::HashMap;

fn hashmap5() {
    let vote = vec!["Alice", "Bob", "Alice", "Charlie", "Bob", "Alice"];
    let mut vote_map: HashMap<&str, usize> = HashMap::new();
    
    for &name in &vote {
        *vote_map.entry(name).or_default() += 1;
    }

    let result = vote_map.iter().max_by_key(|&(_, score)| score).unwrap();
    //let result = vote_map.iter().max_by_key(|&(name, _)| name).unwrap(); これだと、("Charlie", 1)が出力されてしまう
    println!("{:?}", result);
}

fn hashmap6() {
    // 価格表の作成（ここはそのままコピーして使ってOKです）
    let mut prices: HashMap<&str, u32> = HashMap::new();
    prices.insert("apple", 100);
    prices.insert("banana", 200);

    // 買い物リスト
    let cart = vec!["apple", "orange", "banana", "apple"];

    let mut total: u32 = 0;
    for item in cart {
        //Some()の書き方に慣れないなあ
        //Some(変数)は、変数が要素を持っているかNoneのどちらかのOptionを持つ
        //なので、ifやmatchで条件分岐する必要がある
        if let Some(value) = prices.get(item) {
            total += value;
        } else {
            continue;
        }
    }
    //forのコードは、以下の書き方で1行で書ける
    //let total: u32 = cart.iter().filter_map(|&i| prices.get(i)).sum();

    println!("{}", total);
}

fn hashmap7() {
    let scores = vec![
        ("Alice", 80),
        ("Bob", 90),
        ("Alice", 85),
        ("Charlie", 70),
        ("Bob", 95),
    ];

    let mut score_map: HashMap<&str, Vec<u32>> = HashMap::new();

    for (name, score) in scores {
        score_map.entry(name).or_default().push(score);
    }
    
    for (s, i) in score_map {
        println!("{} -> {:?}", s, i);
    }
}

fn main() {
    hashmap5();
    hashmap6();
    hashmap7();
}
//0152_HashMap_test_3本