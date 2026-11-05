use std::{io::{self, Read}, vec};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let names: Vec<String> = iter.by_ref().take(n).map(|x| x.to_string()).collect();
    let votes: Vec<String> = iter.by_ref().take(m).map(|x| x.to_string()).collect();
    let mut results = vec![0u32; n];

    'vote: for v in &votes {
        // let mut exact = None; ←AIにこの変数をヒントでもらったが、使い方がわからなかったのでコメントアウトした
        let mut similar = Vec::new();

        for (idx, name) in names.iter().enumerate() {
            if v.len() != name.len() {
                continue;
            }

            let diff = v.chars()
                .zip(name.chars())
                .filter(|(a, b)| a != b)
                .count();

            match diff {
                0 => {
                    results[idx] += 1;
                    continue 'vote;
                }
                1 => similar.push(idx),
                _ => (),
            }
        }

        if similar.len() == 1 {
            results[similar[0]] += 1;
        }
    }

    for ans in results {
        println!("{ans}");
    }
}
//0230_abc480_B

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();
//     let mut iter = input.trim().split_whitespace();
//     let n: usize = iter.next().unwrap().parse().unwrap();
//     let m: usize = iter.next().unwrap().parse().unwrap();

//     let names: Vec<String> = iter.by_ref().take(n).map(|x| x.to_string()).collect();
//     let votes: Vec<String> = iter.by_ref().take(m).map(|x| x.to_string()).collect();
//     let mut results:Vec<u32> = Vec::with_capacity(n);

//     let mut similer_name: Vec<u32> = Vec::with_capacity(n);
//     for (idx, name) in names.iter().enumerate() {

//         for v in &votes {
//             if name.len() != v.len() {
//                 continue;
//             }

//             let diff = name.chars()
//                 .zip(v.chars())
//                 .filter(|(a, b)| a != b)
//                 .count();

//             match diff {
//                 0 => results[idx] += 1,
//                 1 => similer_name[idx] += 1,
//                 _ => (),
//             }
//         }
//     }

//     let mut idx = 0;

//     for i in similer_name {

//         if i == 1 {
//             results[idx] += 1;
//         }

//         idx += 1;
//     }

//     for ans in results {
//         println!("{ans}");
//     }
// }