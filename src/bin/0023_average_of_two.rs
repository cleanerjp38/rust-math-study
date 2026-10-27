mod gateway;
use gateway::Gateway;

struct Calculator {
    a: u32,
    b: u32,
}

impl Calculator {
    fn new() -> Self {
        let mut gateway = Gateway::new();
        let x = gateway.next();
        let y = gateway.next();
        Self { a: (x), b: (y) }
    }

    fn calculator_average(&self) -> u32 {
        (self.a + self.b + 1 )/ 2
    }

    fn result(&self) {
        println!("結果：{}", self.calculator_average())
    }
}

fn main() {
    let caluculator = Calculator::new();
    caluculator.calculator_average();
    caluculator.result();
}
//0023_average_of_two
