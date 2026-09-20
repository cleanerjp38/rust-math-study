use std::vec;

fn bit_search3() {
    let a = vec![10, 20, 30];
    let n = a.len();

    for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum+= a[i];
            }
        }
        println!("{}", sum);
    }
}

fn bit_search4() {
    let a = vec![3, 5, 8, 10];
    let target = 13;
    let n = a.len();

    for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += a[i];
            }
        }
        if target == sum {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn bit_search5() {
    let a = vec![2, 4, 5, 7];
    let limit = 10;
    let n = a.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += a[i];
            }
        }

        if sum <= limit {
            ans = ans.max(sum);
        }
    }

    println!("{}", ans);
}

fn main() {
    bit_search3();
    println!("");
    bit_search4();
    println!("");
    bit_search5();
    println!("");

}
//0190_bit全探索_練習_3本