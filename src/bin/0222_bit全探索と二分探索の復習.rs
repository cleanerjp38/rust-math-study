fn bit_research1(a: [u32; 3]) {
    let n = a.len();

    for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += a[i];
            }
        }

        println!("{sum}");
    }
}

fn bit_research_check() {
    let a = [1, 2, 3];
    let n = a.len();

    for bit in 0..(1 << n) {
        print!("{bit}");
        println!(" {:04b}", bit);
    }
}



fn two_pointer1() {
    let a = [2, 3, 5, 1, 4, 2, 6];
    let k = 10;

    let mut left = 0;
    let mut sum = 0;
    let mut ans = 0;

    for right in 0..a.len() {
        sum += a[right];

        // while sum > k && left < right { left < rightだと、要素1個だけの場合を想定できない
        while sum > k {
            sum -= a[left];
            left += 1;
        }
        let count = right - left + 1;

        ans = ans.max(count);
    }

    println!("{ans}");
}


fn main() {
    bit_research1([3, 5, 7]);
    println!("");
    two_pointer1();
    println!("");
    bit_research_check();
}
//0222_bit全探索と二分探索の復習
