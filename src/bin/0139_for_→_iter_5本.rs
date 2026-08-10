fn for1() -> u32 {
    let mut count = 0;
    
    for _ in 0..100 {
        count += 1;
    }

    let for1 = count;
    for1
}

fn iter1() -> u32 {
    //わざわざ変数を返さずとも、この1行で数値は出るのか。すごい
    (0..100).count() as u32
}

fn for2() -> f32 {
    let people =100;
    let days_in_year = 365.0;
    let mut probability = 1.0;
    for i in 0..people {
        probability *= (days_in_year - i as f32) / days_in_year;
    }

    probability
}

fn iter2() -> f32 {
    let people = 100;
    let days_in_year = 365.0_f32;
    let probability: f32 = (0..people)//凄い、(0..変数)って書くとRange<i32>になっている
        //map()ってforやifの条件をかなりそのまま書いて良いんだね
        .map(|i| (days_in_year - i as f32) / days_in_year)
        //product()は、イテレータで流れてくる要素を全て掛け合わせる。ということは使えるのは数値限定か？
        .product();

    probability
}

fn for3() -> u32 {
    let mut count = 0;

    for i in 0..=100 {
        if i % 2 == 0 {
            count += 1;
        }
    }
    count
}

//iter1の応用版
fn iter3() -> u32 {
    (0..=100)
    .filter(|i| i % 2 == 0)
    .count() as u32
}

fn for4() -> i32 {
    let mut score = 0;
    let symbols = "+-+-++++-+-++++-+-++++";

    for sym in symbols.chars() {
        if sym == '+' {
            score += 1;
        } else if sym == '-' {
            score -= 1;
        } else {
            score += 0;
        }
    }
    score
}

fn iter4() -> i32 {
    let symbols = "+-+-++++-+-++++-+-++++";

    symbols
        .chars()
        //mapの中でmatchで条件分岐させれば良いのか
        .map(|sym| match  sym {
            '+' => 1,
            '-' => -1,
            _ => 0,
        })
        //流れてくるイテレータを足し合わせる
        .sum()
}

fn for5() -> Vec<String> {
    let mut ans_vec = Vec::new();

    for i in (1..=10).rev() {
        let string_n = i.to_string();
        ans_vec.push(string_n);
        if i != 1 {
            ans_vec.push(",".to_string());
        }
    }
    ans_vec
    //出力が
    //["10", ",", "9", ",", "8", ",", "7", ",", "6", ",", "5", ",", "4", ",", "3", ",", "2", ",", "1"]
    //になってしまった
}

fn iter5() -> String {
    let ans_vec = (1..=10)
        .rev()
        //map()はほんとになんでも出来るな笑
        .map(|i| i.to_string())
        //collect()ってparse()みたいな書き方するんだね
        .collect::<Vec<_>>()
        .join(",");

    ans_vec
}


fn main() {
    println!("{}   {}   {}   {}   {}   {}   {}   {}   {:?}   {}", for1(), iter1(), for2(), iter2(), for3(),iter3(), for4(), iter4(), for5(), iter5());
}
//0139_for_→_iter_5本