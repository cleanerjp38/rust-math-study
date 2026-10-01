mod gateway;
use gateway::Gateway;

struct Bill {
    x: u32,//品代
    n: u32,//人数
    s: u32,//閾値
}

impl Bill {
    fn calculate_price(&self) -> u32 {
        let base_total = self.x * self.n;
       
        if base_total > self.s {//合計金額が閾値を超えたら超過分を10％加算
            base_total + (base_total - self.s)/10
        } else {
            base_total
        }
    }
}

fn main() {
    let mut gateway = Gateway::new();
   
    let x = gateway.next();//これらの数値の埋め込みは関所内でできると、よりmainがスッキリする
    let n = gateway.next();
    let s = gateway.next();
    let bill = Bill{x, n, s};//my_billのほうがRustらしいのか？
   
    println!("合計{}円", bill.calculate_price());
}
//0020_飲み代料金