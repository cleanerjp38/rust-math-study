mod gateway;

use gateway::Gateway;

struct Pair {
    a: u32,
    b: u32,
}

impl Pair {
    fn product(&self) -> u32 {
        self.a * self.b
    }
}

enum Parity {
    Even,
    Odd,
}

impl Parity {
    fn new(n: u32) -> Self {
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
//0021_Parity_&self