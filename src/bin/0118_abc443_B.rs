use std::io;


fn get_nk() -> (f64, f64) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nk = buf.trim().split_whitespace();
    let n: f64 = nk.next().unwrap().parse().unwrap();
    let k: f64 = nk.next().unwrap().parse().unwrap();

    (n, k)
}

//N+(N+1)+(N+2)..+(N+x)>=kをxの二次不等式として解いた
fn cal_nk(n: f64, k: f64) -> u64 {
    let a: f64 = (2.0 * n - 1.0) * (2.0 * n - 1.0) + 8.0 * k;
    let a_root = a.sqrt();
    let result_f: f64 = ((-2.0 * n - 1.0) + a_root) / 2.0;
    //ceil()によって、小数点以下を切り上げる
    let result_u = result_f.ceil() as u64;
    //ただu64にすると、切り捨てにより正解ではなくなる
    //let result_u = result_f as u64;
    result_u
}

fn main() {
    let (n, k) = get_nk();
    println!("{}", cal_nk(n, k));
}
//0118_abc443_B