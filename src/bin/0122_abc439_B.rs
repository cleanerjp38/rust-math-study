use std::io;

fn get_n() -> u32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n:u32 = buf.trim().parse().unwrap();
    n
    //nを作らなくても、bufをそのまま放り込んでもいいっぽい
    //buf.trim().parse().unwrap();
}

//数値を分解して、それぞれを2乗する関数を作る
fn formula(n: u32) -> u32 {
    let n_st: String = n.to_string();
    let mut sum = 0;

    //例によって、forでなくイテレータで計算できるっぽい
    //n.to_string()
        //.chars()
        //.map(|c| c.to_digit(10).unwrap())
        //.map(|d| d * d) map()の使い方が理解できなくなる。すごすぎい
        //.sum()
    for c in n_st.chars() {
        let c_num = c.to_digit(10).expect("Is Not a Number");
        sum += c_num * c_num;
    }
    sum
}

fn happy_number() {
    let mut n = get_n();
    let mut stock: Vec<u32> = Vec::new();
    loop {
        let sum = formula(n);

        match sum {
            1 => {
                println!("Yes");
                break;
            }
            //match内では関数やメソッド呼び出しは書けない
            //stock.iter().filter(|n| n.contain(sum)) => {
            //_ ifって書き方ならいいのか。あとcontain(&sum)で配列の中にあるか調べられる
            _ if stock.contains(&sum) => {
                println!("No");
                break;
            }
            _ => {
                stock.push(sum);
                n = sum;
            }
        }
    }
} 

fn main() {
    happy_number();
}
//0122_abc439_B