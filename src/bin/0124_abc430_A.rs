use std::io;

fn get_abcd() -> (u32, u32, u32, u32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut abcd = buf.trim().split_whitespace();
    let a: u32 = abcd.next().unwrap().parse().unwrap();
    let b: u32 = abcd.next().unwrap().parse().unwrap();
    let c: u32 = abcd.next().unwrap().parse().unwrap();
    let d: u32 = abcd.next().unwrap().parse().unwrap();

    (a, b, c, d)
}

fn candy_law(a:u32, b:u32, c:u32, d:u32) {
    if a <= c && d <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let (a, b, c, d) = get_abcd();
    candy_law(a, b, c, d);
}
//0124_abc430_A