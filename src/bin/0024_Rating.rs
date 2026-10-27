mod gateway;
use gateway::Gateway;

struct Rating {
    r: i32,
    g: i32,
}

impl Rating {
    fn new() -> Self {
        let mut gateway = Gateway::new();
        let x = gateway.next();
        let y = gateway.next();
        Self {r: (x), g: (y)}
    }

    fn required_performance(&self) -> i32 {
        2 * self.g - self.r
    }

    fn result(&self) {
        println!("{}", self.required_performance())
    }
}



fn main() {
    let rating = Rating::new();
    rating.required_performance();
    rating.result();
}
//0024_Rating?
