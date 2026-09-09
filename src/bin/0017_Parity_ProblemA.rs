mod gateway;

use gateway::Gateway;

struct Pair {
    a: i32,
    b: i32,
}

impl Pair {
    fn product(&self) -> i32 {
        self.a * self.b
    }
}

enum Parity {
    Even,
    Odd,
}

impl Parity {
    fn new(n: i32) -> Self {
        if n % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        }
    }

    fn format(&self) -> &str {
        match self {
            Parity::Even => "Even",
            Parity::Odd => "Odd",
        }
    }
}

fn main() {
    let mut gateway = Gateway::new();
    let pair = Pair {
        a: gateway.next(),
        b: gateway.next(),
    };

    let product = pair.product();
    let result = Parity::new(product);
    println!("{}", result.format());
}
//0017_Parity_ProblemA