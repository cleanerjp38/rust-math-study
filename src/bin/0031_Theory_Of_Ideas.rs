// 1. イデア（trait）
// 型が満たすべき振る舞いを定義する
trait Form {
    fn esssence(&self) -> &str;
}

// 2. 実体（struct）
// 具体的なデータを所有する
struct RealObject {
    name: String,
}

impl Form for RealObject {
    fn esssence(&self) -> &str {
        &self.name
    }
}

// 3. 投影（影）
// 実体への参照を保持する
struct Shadow<'a> {
    source: &'a dyn Form,
}

impl<'a> Shadow<'a> {
    fn look_at(&self) {
        println!(
            "囚人が見ている影： {}のぼんやりとした輪郭",
            self.source.esssence()
        );
    }
}

fn main() {
    let wooden_bed = RealObject {
        name: String::from("木椅子"),
    };

    let shadow = Shadow {
        source: &wooden_bed,
    };

    shadow.look_at();
}
//0031_Theory_Of_Ideas