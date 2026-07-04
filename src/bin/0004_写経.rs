use std::io;

struct NumberPair{
    a:u32,
    b:u32,
}

impl NumberPair{
    fn check_parity(&self) -> String{
        let product = self.a * self.b;
        
        match product % 2{
            0 => String::from("Even"),
            _ => String::from("Odd")
        }
    }
}

fn main(){
    let pair = get_gateway_data();
    let result = pair.check_parity();
    
    println!("{}", result);
}

fn get_gateway_data() -> NumberPair{
    let mut buf =String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    let v: Vec<u32> = buf.trim().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    NumberPair { a: v[0], b: v[1]
    }
}
//0004_写経