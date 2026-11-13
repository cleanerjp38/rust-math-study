use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut result = Vec::new();
    for c in s {
        result.push(c);
        result.push('o');
    }

    result.pop();
    let ans: String = result.iter().collect();

    println!("{ans}");
}
//0249_abc475_A

// sをVec<char>にする
// forループでひとつずつ受け取り、
// result.push(c);
// result.push('o');
// ループ後に、result.remove(result.len())をする　←pop()は、配列の一番うしろを取り出すからそっちのほうがよい。あと、len()するなら-1が必要
// ans: String = result.collect;　で、ansを出力 ←iter().collect()と各必要がある