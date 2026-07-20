struct Book{
    title: String
}

struct Library{
    book: Book
}

impl Library{
    fn maintain(&mut self){
        println!("図書館員: 本のページを修復します。");
    }
}

fn main() {
    let mut bunkyo_lib = Library{
        book: Book{title: String::from("構造再帰型プログラミングの実践")},
    };
    
    println!("--- 窓口にMiyaが登場 ---");
    
    {
    // 1. Miyaが本を借りる（不変の借用）
        let borrowed_book = &bunkyo_lib.book;
        println!("Miya: 「{}」を熟読中...", borrowed_book.title);
    }// ここでMiyaが本を返却（スコープ終了）
    
    // 3. 返却された後なら、メンテナンスができる
    bunkyo_lib.maintain();
    println!("--- 全ての処理が論理的に正しい順番で行われました ---");
}
//0008_図書館と借用