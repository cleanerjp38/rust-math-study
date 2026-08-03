use std::io::{self, BufRead};

fn common_point() {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let case_line = lines.next().unwrap().unwrap();
        let mut case = case_line.trim().split_whitespace();
        let x1: i64 = case.next().unwrap().parse().unwrap();
        let y1: i64 = case.next().unwrap().parse().unwrap();
        let r1: i64 = case.next().unwrap().parse().unwrap();
        let x2: i64 = case.next().unwrap().parse().unwrap();
        let y2: i64 = case.next().unwrap().parse().unwrap();
        let r2: i64 = case.next().unwrap().parse().unwrap();
    
        let x_m = x1 - x2;
        let y_m = y1 - y2;
        let r_m = r1 - r2;
        let r_p = r1 + r2;

        if r_m * r_m <= (x_m * x_m) + (y_m * y_m) && (x_m * x_m) + (y_m * y_m) <= r_p * r_p {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    common_point();
}
//0131_abc460_B_2回目