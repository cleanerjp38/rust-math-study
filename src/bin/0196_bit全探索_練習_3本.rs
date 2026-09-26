fn bit_search15() {
    let lamps = [
        vec![0, 1],       // 電球0
        vec![1, 2],       // 電球1
        vec![0, 2, 3],    // 電球2
    ];
    let mut count =0;

    for bit in 0..(1 << 4) {
        let mut flag = 0;
        for lamp in &lamps {
            let mut lights = 0;
            for switch in lamp {
                
                if bit & (1 << switch) != 0 {
                    lights += 1;
                }
            }
            if lights % 2 == 0 {
                flag += 1;
            }
        }
        if flag == 3 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn bit_search15_continue() {
        let lamps = [
        vec![0, 1],       // 電球0
        vec![1, 2],       // 電球1
        vec![0, 2, 3],    // 電球2
    ];
    let mut count =0;

    for bit in 0..(1 << 4) {
        let mut flag = 0;
        for lamp in &lamps {
            let mut lights = 0;
            for switch in lamp {
                if bit & (1 << switch) != 0 {
                    lights += 1;
                }
            }
            if lights % 2 != 0 {
                continue;
            }
            flag += 1;
        }
        if flag != 3 {
            continue;
        }
        count += 1;
    }

    println!("{}", count);
}

fn bit_search16() {
    let scores = [8, 6, 10, 7, 9];
    let ng_pairs = [
        (0, 1),
        (1, 2),
        (2, 4),
    ];
    let n = scores.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum =0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += scores[i];
            }
        }

        if bit & (1 << 0) != 0 && bit & (1 << 1) != 0 {
            continue;
        }

        if bit & (1 << 1) != 0 && bit & (1 << 2) != 0 {
            continue;
        }

        if bit & (1 << 2) != 0 && bit & (1 << 4) != 0 {
            continue;
        }

        ans = ans.max(sum);
    }

    println!("{}", ans);
}

fn main() {
    bit_search15();
    bit_search15_continue();
    bit_search16();
}
//0196_bit全探索_練習_3本