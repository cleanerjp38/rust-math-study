use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    //idxが1始まりのため、m+1となっている
    let mut weight_sum: Vec<u32> = vec![0; m + 1];
    let mut count_vec: Vec<u32> = vec![0; m + 1];

    for _ in 0..n {
        let idx: usize = iter.next().unwrap().parse().unwrap();
        let b: u32 = iter.next().unwrap().parse().unwrap();

        weight_sum[idx] += b;
        count_vec[idx] += 1;
    }

    for i in 1..=m {
        let ans: f32 = weight_sum[i] as f32 / count_vec[i] as f32;
        //{:.小数点以下の桁}で、必要な小数点以下を出力できる
        println!("{:.10}", ans);
    }
}
//0178_abc434_B
//Aiをインデックス番号として使う
//1．n,mを受け取る 
//2．aを使って、bと個数をそれぞれ一次配列に足していく 
//3．bと個数のインデックス番号が同じ物を割る。{:.5} が小数点以下5桁 
//4．3の作業中に割った結果を出力する