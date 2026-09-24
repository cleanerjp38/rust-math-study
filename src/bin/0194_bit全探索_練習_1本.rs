fn bit_search13() {
    let a = [2, 3, 5, 8];
    let n = a.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;
        let mut one = false;
        let mut valid = true;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                 if one {
                    valid = false;
                 }
                 sum += a[i];
                 one = true;
            } else {
                one = false;
            }
        }
        if valid {
            ans = ans.max(sum);
        }
    }

    println!("{}", ans);
}

fn main() {
    bit_search13();
}
//0194_bit全探索_練習_1本