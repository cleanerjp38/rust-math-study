use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut hb = buf.trim().split_whitespace();
    //ここ、手癖でu32と書いてエラー出た
    let h: i32 = hb.next().unwrap().parse().unwrap();
    let b: i32 = hb.next().unwrap().parse().unwrap();

    let weight = h - b;
    if weight <= 0 {
        println!("0");
    } else {
        println!("{}", weight);
    }
}
//0123_abc431_A