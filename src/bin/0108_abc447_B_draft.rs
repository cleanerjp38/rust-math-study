use std::io::{self, BufRead};

fn get_s() -> String {
    let std = io::stdin();
    let b = io::BufReader::new(std.lock());
    let mut l = b.lines();
    let ln = l.next().unwrap().unwrap();
    let st = ln.trim().to_string();

    st
}

fn out() -> String {
    let st = get_s();
    let mut i_c:Vec<u32> = vec![0; 26];

    for c in st.chars() {
        let index = (c as usize) - ('a' as usize);
        i_c[index] += 1;
    }

    //let max = i_c.iter().max().unwrap();
    let max = *i_c.iter().max().unwrap();
    //let res_i = i_c.iter().filter(|&s| s != max).collect();
    //res_i
    let res_i: String = st.chars().filter(|&c| {
        let index = (c as usize) - ('a' as usize);
        i_c[index] != max
    })
    .collect();
    res_i
}

fn main() {
    println!("{}",out());
}
//0108_abc447_B_draft