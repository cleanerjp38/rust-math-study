mod gateway;

use gateway::Gateway;

struct Sequence {
    start: i32,
    end: i32,
    step: i32,
}

impl Sequence {
    fn display(&self) {
        let mut current = self.start;
        while current <= self.end {
            print!("{}", current);
            current += self.step;
        }
        println!();
    }
}

fn main() {
    let mut gateway = Gateway::new();

    let a: i32 = gateway.next();
    let b: i32 = gateway.next();
    let d: i32 = gateway.next();
    let seq = Sequence {
        start: a,
        end: b,
        step: d,
    };

    seq.display();
}
//0016_sequence