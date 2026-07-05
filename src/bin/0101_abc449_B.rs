use std::io::{self, BufRead};

struct Chocolate {
    h: u32,
    w: u32,
    q: usize,//forでqueryの行数を数えてくれたから、qは使わなかった
    query: Vec<Vec<u32>>,//来るクエリは2要素と分かっているから、Vec<(u32, u32)>で作ったほうがメモリ効率が良いらしい
}

fn get_query() -> Chocolate {
    let buf = io::BufReader::new(io::stdin().lock());
    //ここ、io::BufReader::new(io::stdin().lock())だと環境によってはエラーになるらしい
    //let stdin = io::stdin();
    //let buf = io::BufReader::new(stdin.lock());
    //だと、より安全だそうだ

    //一気に標準入力の処理をして一つの変数に入れても、所有権によって変数が消費されてしまう
    //なので、変数を分けて処理をし、「lines」という入力の受け皿は消費させないようにしている
    //let mut line_hwq = buf.lines().next().expect("INPUT NULL").expect("INPUT Error").trim().split_whitespace();
    let mut lines = buf.lines();
    let first_line = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let mut hwq = first_line.trim().split_whitespace();

    let h: u32 = hwq.next().unwrap().parse().unwrap();
    let w: u32 = hwq.next().unwrap().parse().unwrap();
    let q: usize = hwq.next().unwrap().parse().unwrap();

    //vec![]で箱を作ったつもりだが、このマクロは箱の作成かつ具体的な要素の数値も入れる機能だった
    //なので、vec![]で作った後にforで回すと、q×2行の配列ができてしまう
    //let mut query: Vec<Vec<u32>> = vec![vec![0; 2]; q];
    let mut query: Vec<Vec<u32>> = Vec::with_capacity(q);
    for _ in 0..q {
        //上で変数を分けておいたので、ここでlinesを使えている
        let line = lines.next().unwrap().unwrap();
        let row: Vec<u32> =  line.split_whitespace().map(|i| i.parse().unwrap()).collect();
        query.push(row);
    }

    Chocolate { h, w, q, query }
}

impl Chocolate {
    fn is_eaten(&self) {
        //----ここからはAIに教わりつつ書いた----
        let mut current_h = self.h;
        let mut current_w = self.w;

        //ここで&self.queryを参照すると&Vec<u32>の変数が生まれることに驚いた。便利だ
        for q_vec in &self.query {
            //クエリのタイプ
            let q_type = q_vec[0];
            //食べる個数
            let val = q_vec[1];

            //ここでifでなくmatchを使えるのが良いね
            //q_typeは2種類しかないので、matchだと書きやすい
            match q_type {
                1 => {
                    let eaten = val * current_w;
                    println!("{}", eaten);

                    current_h -= val;
                }
                2 => {
                    let eaten = val * current_h;
                    println!("{}", eaten);
                    current_w -= val;
                }
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let chocolate = get_query();
    chocolate.is_eaten();
}
//0101_abc449_B