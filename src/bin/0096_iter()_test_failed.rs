use std::io;

fn count_char() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut input_s: String = buffer.trim().parse().unwrap();
    let count_iter = 0;//mutがない
    //Stringに対してはiter()は使えない
    //今回のコードなら、chars()を使い、文字列を分解する
    for _ in &input_s.iter() {
        count_iter += 1;
    }

    if count_iter == input_s.len() {
        println!("Yes");
    }

    println!("{}", count_iter);
}

fn main() {
    count_char();
}
//0096_iter()_test_failed