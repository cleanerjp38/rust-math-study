use proconio::input;

fn main() {
    input! {
        s: char,
    }

    match s {
        'B' => println!("Y"),
        'Y' => println!("R"),
        'R' => println!("B"),
        _ => (),
    }
}
//0236_abc477_A