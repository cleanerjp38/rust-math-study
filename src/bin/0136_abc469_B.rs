use std::io;

fn get_seats() -> (usize, String) {
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).unwrap();

    let mut input_s = String::new();
    io::stdin().read_line(&mut input_s).unwrap();

    let n: usize = input_n.trim().parse().unwrap();
    let s = format!("x{}x", input_s.trim());

    (n, s)
}

fn check_isolated() -> u32 {
    let (n, s) = get_seats();

    let s_vec: Vec<char> = s.chars().collect();
    let isolated_count = s_vec
        .windows(3)
        .filter(|window| window == &['x', 'x' ,'x'])
        .count();

    isolated_count as u32
}

fn main() {
    let count = check_isolated();
    println!("{}", count);
}
//0136_abc469_B