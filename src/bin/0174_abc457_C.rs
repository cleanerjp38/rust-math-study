use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nk = first_line.trim().split_whitespace();
    let n: usize = nk.next().unwrap().parse().unwrap();
    let mut k: u64 = nk.next().unwrap().parse().unwrap();

    let mut l_vec: Vec<u64> = Vec::with_capacity(n);
    let mut a_vec: Vec<Vec<u64>> = Vec::with_capacity(n);
    for _ in 0..n {
        let next_line = lines.next().unwrap().unwrap();
        let mut la = next_line.trim().split_whitespace();
        let l: u64 = la.next().unwrap().parse().unwrap();

        let mut row: Vec<u64> = Vec::with_capacity(l as usize);
        for _ in 0..l {
            let a: u64 = la.next().unwrap().parse().unwrap();
            row.push(a);
        }
        a_vec.push(row);
        l_vec.push(l);
    }

    let c_line = lines.next().unwrap().unwrap();
    let mut c_split = c_line.trim().split_whitespace();

    for i in 0..n {
        let c:u64 = c_split.next().unwrap().parse().unwrap();
        let l = l_vec[i];
        let block_len = l * c;

        //if k < block_len { 「<=」でないとk=block_lenのときにパニックになる
        if k <= block_len {
            let idx = (k - 1) % l;//ここ、後にusizeにするならk＝0の場合を加味したほうが安全
            println!("{}", a_vec[i][idx as usize]);
            return;
        } else {
            k -= block_len;
        }
    }
}
//0174_abc457_C
//まず、n,k,a_vec,l_vecを作る
//cを流しつつ、l*c>kとなるcを探す
//a_vec[i][(k-1)%l]を出力
//c_Vecを作って、更に[(k-1)%l]でなく[k-(c1~c(i-1))]をやろうとしていたが、AIが良い方法を教えてくれた
//BufReadでやってみた