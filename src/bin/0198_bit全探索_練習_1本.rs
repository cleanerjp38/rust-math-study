fn main() {
    let scores = [8, 6, 10, 7, 9];
    let ng_pairs = [
        (0, 1),
        (1, 2),
        (2, 4),
    ];
    let n = scores.len();
    let mut ans = 0;

    'bits: for bit in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += scores[i];
            }
        }

        for (a, b) in ng_pairs {
            if bit & (1 << a) != 0 && bit & (1 << b) != 0 {
                continue 'bits;
            }
        }

        ans = ans.max(sum);
    }

    println!("{}", ans);
}
//0198_bit全探索_練習_1本