use std::io;

enum Slot {
    Occupied, // 1：ある
    Empty,    // 0：ない
}

struct Board {
    slots: Vec<Slot>,
}

fn get_board_gateway() -> Board {
    // 外界から生の文字列を読み取る
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("読み込み失敗");

    // 文字列を1文字ずつSlotへ変換する
    let slots = input
        .trim()
        .chars()
        .map(|c| {
            match c {
                '1' => Slot::Occupied,
                _ => Slot::Empty,
            }
        })
        .collect();

    Board { slots }
}

fn main() {
    let board = get_board_gateway();

    // 今後、boardの要素を数える処理を実装する
}

//0028_Unfinished001