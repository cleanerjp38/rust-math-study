fn bit_search1() {
    let n = 3;

    for bit in 0..(1 << n) {
        println!("{}", bit);
    }
}

fn bit_search2() {
    let n = 3;

    for bit in 0..(1 << n) {
        println!("{}", bit);
        for i in 0..n {
            if bit & (1 << i) != 0 {
                println!(" {}番目を選ぶ", i);
            }
        }
    }
}

fn main() {
    bit_search1();
    println!("");
    bit_search2();
    println!("");
}
//0189_bit全探索_練習_2本