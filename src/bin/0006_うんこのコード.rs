// 1. 食べたものを定義（これは関所の素材）
enum Food {
    HealthyMeal,
    JunkFood,
}

// 2. 「腸」の状態そのものを型で表現する
// 状態ごとに持たせる情報を変えられるのがRustの強み
enum Intestine {
    Normal,
    Irritated { pain_level: u32 }, // ゲリの時は「痛み」の情報も持たせる
}

impl Intestine {
    // 食べるという行為で、新しい「腸の状態」を返す
    fn ingest(self, food: Food) -> Self {
        match food {
            Food::HealthyMeal => Intestine::Normal,
            Food::JunkFood => Intestine::Irritated { pain_level: 10 },
        }
    }

    // 出すという行為。自分（腸）の状態を見て、うんこを出す。
    fn poop(&self) {
        match self {
            Intestine::Normal => println!("（コロン）快便！"),
            Intestine::Irritated { pain_level } => {
                println!("（ブリブリ）痛ぇ！痛みレベル: {}！", pain_level)
            }
        }
    }
}

fn main() {
    // 10年後もバグらない、流れるような記述（メソッドチェーン）
    let my_gut = Intestine::Normal;
    
    my_gut
        .ingest(Food::JunkFood) // ジャンクフードを食べて、新しい状態の腸になる
        .poop();                // そのまま出す
}
//0006_うんこのコード