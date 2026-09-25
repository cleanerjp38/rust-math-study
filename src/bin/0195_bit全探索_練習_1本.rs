fn bit_search14_failed() {
    let scores = [8, 6, 9, 5, 7];
    let n = scores.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;
        let mut count= 0;
        let mut zero = false;
        let mut two = false;
        let mut four = false;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                if count == 3 {
                    break;
                }
                if i == 0 {
                    zero = true;
                }
                if i == 1 && zero {
                    break;
                }
                if i == 2 {
                    two = true;
                }
                if i == 4 {
                    four = true;
                }
                if two && !four {
                    break;
                }
                sum += scores[i];
                count += 1;
            }
        }
        ans = ans.max(sum);
    }

    println!("{}", ans);
}

fn bit_search14() {
    let scores = [8, 6, 9, 5, 7];
    let n = scores.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;
        let mut count = 0;
        for i in 0..n {
            if bit & (1 << i) != 0{
                sum += scores[i];
                count += 1;
            }
        }

        if count != 3 {
            continue;
        }

        if bit & (1 << 0) != 0 && bit & (1 << 1) != 0 {
            continue;
        }

        if bit & (1 << 2) != 0 && bit & (1 << 4) == 0 {
            continue;
        }

        ans = ans.max(sum);
    }
    println!("{}", ans);
}

fn main() {
    bit_search14();
}
//0195_bit全探索_練習_1本