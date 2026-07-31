// Step 2: 「外界の泥」を入れない聖域（main）のための、純粋なデータ構造

// 名詞1: リザード（進化前）
#[derive(Debug)] // 中身を表示できるようにする魔法
struct Charmeleon {
    name: String,
    level: u32,
    hp: u32,
}

// 名詞2: リザードン（進化後）
#[derive(Debug)]
struct Charizard {
    name: String,
    level: u32,
    hp: u32,
    can_fly: bool, // リザードンになって手に入れた「空を飛ぶ」能力
}

// Step 3: implによる「物語（状態の変化）」の記述

impl Charmeleon {
    // 振る舞い1: リザードを生み出す（new）
    fn new(name: &str, level: u32) -> Self {
        println!("🔥 {} があらわれた！ (Lv.{})", name, level);
        Self {
            name: name.to_string(),
            level,
            hp: level * 10, // レベルに応じたHP
        }
    }

    // 振る舞い2: 経験を積む（レベルアップ）
    // 自分自身（&mut self）を書き換える借用
    fn train(&mut self) {
        self.level += 1;
        self.hp = self.level * 10;
        println!("✨ {} はトレーニングをした！ Lv.{} になった！ (HP: {})", self.name, self.level, self.hp);
    }

    // ★核心：進化（Evolution）
    // `self`（所有権）を受け取り、Charmeleonを「消費」してCharizardを「生み出す」
    fn evolve(self) -> Charizard {
        println!("🌀 おや…？ {} のようすが…！", self.name);
        println!("🎉 おめでとう！ {} は リザードン に進化した！", self.name);

        // 新しい自分（Charizard）を構築して返す
        // 古い自分（Charmeleon）は、この関数が終わると同時にメモリから消える
        Charizard {
            name: self.name, // 名前を引き継ぐ（所有権の移動）
            level: self.level, // レベルを引き継ぐ
            hp: self.level * 15, // 進化してHPが大幅アップ！
            can_fly: true,    // 翼が生えた！
        }
    }
}

impl Charizard {
    // リザードン専用の振る舞い
    fn fly(&self) {
        if self.can_fly {
            println!("🦅 {} は 大空を飛んだ！ レベル{}の貫禄だ！", self.name, self.level);
        }
    }
}

// 聖域：main関数（ここは物語の上演場所）
fn main() {
    // 1. リザードの誕生
    // `mut` をつけることで、レベルアップ（可変の借用）を許可する
    let mut my_pokemon = Charmeleon::new("リザード", 35);

    // 2. トレーニング（状態の変化：レベルアップ）
    my_pokemon.train(); // Lv.36

    // 3. 進化の時（状態の根本的変化）
    // `my_pokemon`（リザード）の所有権を `evolve` に渡し、
    // 返ってきた `Charizard` を新しい変数 `flying_lizard` で受け取る。
    let flying_lizard = my_pokemon.evolve();

    // ★ここで重要！
    // `my_pokemon`（進化前のリザード）はもう存在しない。
    // コンパイラが「もう使っちゃダメ！」と守ってくれる（Excelの元データ削除みたいなもん）。
    // println!("{:?}", my_pokemon); // ←これのコメントを外すとコンパイルエラーになるぜ

    // 4. 新しい姿での振る舞い
    println!("🔍 進化後の姿: {:?}", flying_lizard);
    flying_lizard.fly();
}
//0009_リザードンのコード