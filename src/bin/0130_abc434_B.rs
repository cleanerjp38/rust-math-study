use std::io::{self, BufRead, };

fn get_birds() -> (usize, Vec<Vec<f64>>) {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nm = first_line.trim().split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: usize = nm.next().unwrap().parse().unwrap();

    let mut ab_vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..n {
        let ab_line = lines.next().unwrap().unwrap();
        let ab: Vec<f64> = ab_line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        ab_vec.push(ab);
    }

    (m, ab_vec)
}

fn weight_average() -> Vec<f64> {
    let (m, ab_vec) = get_birds();
    let mut average_vec: Vec<f64> = Vec::new();

        for i in 1..=m {
            let mut weight_sum = 0.0;
            let mut birds_count = 0.0;

            //for row in 0..ab_vec　では駄目だった。
            //row[0] == iの部分でイテレータがどうのこうのとコンパイラに言われた
            for row in &ab_vec {
                if row[0] == i as f64 {
                    birds_count += 1.0;
                    weight_sum += row[1];
                }
            }
            let average = weight_sum / birds_count;
            average_vec.push(average);
        }
    average_vec
}

fn main() {
    let result = weight_average();
    for r in result {
        //これ、割り切れた場合は小数点以下は表示されないんだよな
        //けど、出力例には「32.00000000000000000000」となっている
        //このコードは間違いなのだろうか？
        println!("{}", r);
    }
}
//0130_abc434_B