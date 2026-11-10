use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut candidates = vec![0; n];

    'vote: for vote in &t {
        let mut similar = Vec::new();

        for (idx, name) in s.iter().enumerate() {
            if vote == name {
                candidates[idx] += 1;
                // break;  break と continue の違いを理解していなかった
                continue 'vote;
            }

            if vote.len() != name.len() {//文字列の長さが一致していないと、zip()をするときに困る
                continue;
            }

            if vote.chars().zip(name.chars()).filter(|(x, y)| x != y).count() == 1 {
                similar.push(idx);
            }
        }

        if similar.len() == 1{
            candidates[similar[0]] += 1;
        }

    }
    
    for ans in candidates {
        println!("{ans}");
    }
}
//0239_abc480_B_2回目
//tが完全一致 -> 一票
//tが部分一致 -> 1文字違いが一人だけ -> 一票
//tが部分一致 -> 1文字違いが2人以上 -> 無効票
//for vote in t 
    //for name in s で、1つの票について、候補者を全探索する
//変数similarで、voteについて1文字違いの候補者がいるかカウントする
