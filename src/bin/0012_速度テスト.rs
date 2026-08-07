use std::time::Instant;

fn main() {
    let mut count: u64 = 0;
    let start = Instant::now();

    for _ in 0..100_000_000 {
        count += 1;
    }

    let duration = start.elapsed();
    print!("結果: {}", count);
    print!("Rustのタイム: {:?}", duration);
}
//0012_速度テスト