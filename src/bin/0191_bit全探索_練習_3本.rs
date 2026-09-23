use std::vec;

fn bit_search6() {
    let prices = vec![3, 4, 6];
    let values = vec![4, 5, 10];
    let budget = 7;

    let n = 3;
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut price_sum = 0;
        let mut value_sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                price_sum += prices[i];
                value_sum += values[i];
            }
        }

        if price_sum <= budget {
            ans = ans.max(value_sum);
        }
    }

    println!("{}", ans);
}

fn bit_search7() {
    let a = vec![3, 5, 2];
    let n = a.len();
    
    for bit in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += a[i];
            } else {
                sum -= a[i];
            }
        }

        println!("{}", sum);
    }
}

fn main() {
    bit_search6();
    println!("");
    bit_search7();
}
//0191_bit全探索_練習_3本