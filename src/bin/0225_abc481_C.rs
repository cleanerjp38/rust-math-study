use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut accounts = HashMap::new();
    let mut hashset= HashSet::new();
    let mut count = 1;

    for _ in 0..q {
        let name: String = iter.next().unwrap().to_string();

        if hashset.insert(name.clone()) {
            accounts.entry(name).or_insert(count);
            println!("{count}");
            count += 1;
        } else {
            let ans = accounts.get(&name).unwrap();
            println!("{ans}");
        }
        //以下なら、HashMapだけでも解けたらしい
        // if let Some(&id) = accounts.get(&name) {
        //     println!("{id}");
        // } else {
        //     accounts.insert(name, count);
        //     println!("{count}");
        //     count += 1;
        // }
    }
}
//0225_abc481_C