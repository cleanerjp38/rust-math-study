enum Item{
    Banana,
    GreenShell,
    Star,
}

struct Wario{
    name:String,
    has_star: bool,
}

impl Wario{
    fn pick_item(&mut self, item: Item){
        match item{
            Item::Star => {
                println!("ワリオ「俺様が無敵だ！！」");
                self.has_star = true
            }
            Item::Banana => {
                println!("ワリオ「バナナか、ケッ！」");
            }
            Item::GreenShell => {
                println!("ワリオ「これを甥っ子にぶつけてやる！」")
            }
        }
    }
    
    fn report(&self){
        if self.has_star{
            println!("現在の状態：無敵のワリオ（最強）")
        }
        else{
            println!("現在の状態：ただのワリオ（甥っ子を追跡中）")
        }
    }
}

fn main(){
    let mut wario = Wario{
        name: "Wario".to_string(),
        has_star: false
    };
    
    let box_item = Item::Star;
    
    wario.report();
    wario.pick_item(box_item);
    wario.report();
}
//0005_ワリオのコード