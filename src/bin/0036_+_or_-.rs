use std::io;

fn get_input() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().chars().collect()//Vecに入れるならこの書き方。(v[0], v[1], v[2])みたいな小分けの袋を準備してなくていい
}

struct MindState {
    symbols: Vec<char>,
}

impl MindState {
    fn new() -> Self {
    Self { symbols: get_input() }//Vecそのまま入れることができている

    }

    fn evuluate(&self) -> i32 {
        let mut score = 0;//初期値を0にしておく

        for sym in &self.symbols {//symってなんだ？文字1文字のことか？
            if sym == &'+' {// *sym == '+'でもいいらしい。参照を実体化させているらしい？
                score += 1;
            } else if sym == &'-' {
                score -= 1
            } else {
                score += 0
            }
        }
        score
    }
}



fn main() {
    let mind_state = MindState::new();
    let result = mind_state.evuluate();
    println!("{}", result);
}
//0036_+_or_-