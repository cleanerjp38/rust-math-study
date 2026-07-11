use std::io::{self, BufRead};

fn read_input_string() -> String {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin.lock());
    let mut lines = reader.lines();
    let raw_line = lines.next().unwrap().unwrap();
    let sanitized_string = raw_line.trim().to_string();

    sanitized_string
}

//返り値を&strにしたら、このfnを書き終えた時にエラーになった
//返り値のときはStringにするほうがいいっぽい
// fn remove_most_frequent_chars() -> &str {
fn remove_most_frequent_chars() -> String {
    let input_string = read_input_string();
    //アルファベット26文字用の空の配列を準備
    let mut char_frequencies: Vec<u32> = vec![0; 26];

    for character in input_string.chars() {
        //Rustの文字は、裏側ですべて固有の番号を持っているらしい
        //なので、forで文字列の文字を一つずつaの番号で引くと、0~25のインデックスができる
        let alphabet_index = (character as usize) - ('a' as usize);
        //でた文字のインデックスを足していく
        char_frequencies[alphabet_index] += 1;
    }

    //ここで*をつけないと、&u32になってしまう
    //&u32だと、input_string.chars().filterで&&&u32とかいうわけわからんことになる
    //let max_frequency = char_frequencies.iter().max().unwrap();
    let max_frequency = *char_frequencies.iter().max().unwrap();
    
    //これだとかえってくるのがu32の配列になる
    // let filtered_string = char_frequencies.iter().filter(|&s| s != max_frequency).collect();
    let filtered_string: String = input_string.chars()
        .filter(|&character| {
            //ここでインデックスをもう一度作る
            let alphabet_index = (character as usize) - ('a' as usize);
            //インデックスに対応した文字を!=で除外する
            char_frequencies[alphabet_index] != max_frequency
        })
        .collect();

    filtered_string
}

fn main() {
    println!("{}", remove_most_frequent_chars());
}
//0108_abc447_B_cleancopy