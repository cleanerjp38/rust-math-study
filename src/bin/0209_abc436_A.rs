use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let x: &str = iter.next().unwrap();
    let y: &str = iter.next().unwrap();
    let os = [("Ocelot", 0), ("Serval", 1), ("Lynx", 2)];

    let x_ver = os.iter().find(|(name, _)| *name == x).unwrap().1;
    let y_ver = os.iter().find(|(name, _)| *name == y).unwrap().1;

    if x_ver < y_ver {
        println!("No");
    } else {
        println!("Yes");
    }
}
//0209_abc436_A