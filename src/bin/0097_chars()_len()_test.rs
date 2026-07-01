use std::io;

fn count_chars() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    //let mut input_s: String = buffer.trim().parse().unwrap();　文字列はparse()では駄目なのか
    //parse()は「別の方への翻訳」なので、同型の型の変換には使えないらしい
    let input_s = buffer.trim().to_string();//to_string()で文字列に変換する
    let mut count_chars = 0;

    for _ in input_s.chars() {
        count_chars += 1;
    }

    if count_chars == input_s.len() {
        println!("Yes");
    }

    println!("{}", count_chars);
    //なんと、chars()では文字数を数えるが、len()ではバイト数を数えるそうだ
    //なので、日本語で入力をすると、Yesが出力されない
    println!("{}", &input_s.len());
}

fn main() {
    count_chars();
}
//0097_chars()_len()_test