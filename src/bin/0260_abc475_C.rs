use proconio::input;

fn main() {
    input! {
        n: usize,
        start: usize,
        limit: u64,
        a: [u64; n - 1],
    }

    let n = n;
    let start = start - 1;

    let mut prefix = vec![0; n];

    prefix[0] = 0;
    for i in 1..n {
        prefix[i] = prefix[i - 1] + a[i - 1];
    }

    let mut ans = 1;

    for left in 0..=start {
        let l_s = prefix[start] - prefix[left];
        
        if l_s <= limit {
            for right in start..n {
                let r_s = prefix[right] - prefix[start];

                if l_s + r_s + l_s.min(r_s) <= limit {
                    let dist = right - left + 1;
                    ans = ans.max(dist);
                }
            }
        }  
    }

    println!("{ans}");
}
//0260_abc475_C