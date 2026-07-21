use std::io;

fn get_nk() -> (u32, u32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nk = buf.trim().split_whitespace();
    let n: u32 = nk.next().unwrap().parse().unwrap();
    let k: u32 = nk.next().unwrap().parse().unwrap();

    (n, k)
}

fn ketawa(n: u32, k: u32) -> u32 {
    let mut count = 0;
    
    for i in 1..=n {
        //ここで数値を文字列に変えている
        let i_string = i.to_string();
        let mut sum = 0;
        //文字列をchars()でイテレータとして流して、1数値ずつ取り扱う
        for j in i_string.chars() {
            //to_digit()は文字をOption<u32>に変える
            //Some(num)でifのほうが安全なのか？
            let num = j.to_digit(10).expect("IS NOT number");
            sum += num;
            }
            if sum == k {
                count += 1;
        }
    }
    count
}

fn main() {
    let (n, k) = get_nk();
    println!("{}", ketawa(n, k))
}
//0117_abc444_B